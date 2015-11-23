use rustc_serialize::json;
use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::io::Read;
use std::u64;

use super::fluid::Fluid;
use super::solid::Solid;
use super::unit::Unit;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Reactor {
	pub fluid_inputs: HashMap<String, (f64, Unit)>,
	pub fluid_outputs: HashMap<String, (f64, Unit)>,
	pub solid_inputs: HashMap<String, u64>,
	pub solid_outputs: HashMap<String, u64>,
	pub power_loss: f64, // joules per reactivity
}

#[derive(Clone, Debug)]
pub struct ReactorContext<'a> {
	pub reactor: &'a Reactor,
	pub size: f64,

	pub fluids: &'a HashMap<String, Fluid>,
	pub solids: &'a HashMap<String, Solid>,

	pub fluid_avail: HashMap<String, f64>,
	pub fluid_space: HashMap<String, f64>,

	pub solid_avail: HashMap<String, u64>,
	pub solid_space: HashMap<String, u64>,

	pub power_avail: f64,
	pub power_space: f64,
}

impl Reactor {
	pub fn react(&self, ctx: &mut ReactorContext) -> f64 {
		let mut round_down = false;
		let mut reactivity = f64::INFINITY;
		for (key, &(need, unit)) in self.fluid_inputs.iter() {
			let need_grams = ctx.fluids[key].grams_from(need, unit);
			match ctx.fluid_avail.get(key) {
				None => return 0.0,
				Some(&avail) => {
					let new_reactivity = avail / need_grams;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		for (key, &need) in self.solid_inputs.iter() {
			round_down = true;
			match ctx.solid_avail.get(key) {
				None => return 0.0,
				Some(&avail) => {
					let new_reactivity = (avail / need) as f64;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		for (key, &(make, unit)) in self.fluid_outputs.iter() {
			let make_grams = ctx.fluids[key].grams_from(make, unit);
			match ctx.fluid_space.get(key) {
				None => return 0.0,
				Some(&space) => {
					let new_reactivity = space / make_grams;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		for (key, &make) in self.solid_outputs.iter() {
			round_down = true;
			match ctx.solid_space.get(key) {
				None => return 0.0,
				Some(&space) => {
					let new_reactivity = (space / make) as f64;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		if self.power_loss > 0.0 {
			let new_reactivity = ctx.power_avail / self.power_loss;
			if new_reactivity < reactivity {
				reactivity = new_reactivity;
			}
		} else if self.power_loss < 0.0 {
			let new_reactivity = ctx.power_space / -self.power_loss;
			if new_reactivity < reactivity {
				reactivity = new_reactivity;
			}
		}
		if round_down {
			reactivity = reactivity.floor();
		}
		if reactivity <= 0.0 {
			return 0.0;
		}
		if reactivity.is_infinite() {
			panic!("Reaction has infinite reactivity");
		}
		for (key, &(need, unit)) in self.fluid_inputs.iter() {
			let need_grams = ctx.fluids[key].grams_from(reactivity * need, unit);
			*ctx.fluid_avail.get_mut(key).unwrap() -= need_grams;
			*ctx.fluid_space.get_mut(key).unwrap() += need_grams;
		}
		for (key, &need) in self.solid_inputs.iter() {
			*ctx.solid_avail.get_mut(key).unwrap() -= reactivity as u64 * need;
			let space = ctx.solid_space.get_mut(key).unwrap();
			if *space != u64::MAX {
				*space += reactivity as u64 * need;
			}
		}
		for (key, &(make, unit)) in self.fluid_outputs.iter() {
			let make_grams = ctx.fluids[key].grams_from(reactivity * make, unit);
			*ctx.fluid_avail.get_mut(key).unwrap() += make_grams;
			*ctx.fluid_space.get_mut(key).unwrap() -= make_grams;
		}
		for (key, &make) in self.solid_outputs.iter() {
			*ctx.solid_avail.get_mut(key).unwrap() += reactivity as u64 * make;
			let space = ctx.solid_space.get_mut(key).unwrap();
			if *space != u64::MAX {
				*space -= reactivity as u64 * make;
			}
		}
		ctx.power_avail -= reactivity * self.power_loss;
		ctx.power_space += reactivity * self.power_loss;
		return reactivity;
	}
}

impl<'a> ReactorContext<'a> {
	pub fn new(reactor: &'a Reactor, size: f64, fluids: &'a HashMap<String, Fluid>, solids: &'a HashMap<String, Solid>) -> ReactorContext<'a> {
		let mut fluid_avail: HashMap<String, f64> = HashMap::new();
		let mut fluid_space: HashMap<String, f64> = HashMap::new();
		let mut solid_avail: HashMap<String, u64> = HashMap::new();
		let mut solid_space: HashMap<String, u64> = HashMap::new();
		for (key, &(need, unit)) in reactor.fluid_inputs.iter() {
			let need_grams = fluids[key].grams_from(size * need, unit);
			fluid_avail.insert(key.clone(), 0.0);
			fluid_space.insert(key.clone(), need_grams);
		}
		for (key, &need) in reactor.solid_inputs.iter() {
			solid_avail.insert(key.clone(), 0);
			if size.is_infinite() {
				solid_space.insert(key.clone(), u64::MAX);
			} else {
				solid_space.insert(key.clone(), size.floor() as u64 * need);
			}
		}
		for (key, &(make, unit)) in reactor.fluid_outputs.iter() {
			let make_grams = fluids[key].grams_from(size * make, unit);
			fluid_avail.insert(key.clone(), 0.0);
			fluid_space.insert(key.clone(), make_grams);
		}
		for (key, &make) in reactor.solid_outputs.iter() {
			solid_avail.insert(key.clone(), 0);
			if size.is_infinite() {
				solid_space.insert(key.clone(), u64::MAX);
			} else {
				solid_space.insert(key.clone(), size.floor() as u64 * make);
			}
		}
		return ReactorContext {
			reactor: reactor,
			size: size,
			fluids: fluids,
			solids: solids,
			fluid_avail: fluid_avail,
			fluid_space: fluid_space,
			solid_avail: solid_avail,
			solid_space: solid_space,
			power_avail: 0.0,
			power_space: size * reactor.power_loss.abs(),
		}
	}

	pub fn react(&mut self) -> f64 { self.reactor.react(self) }
}

pub fn load_reactors(path: &str) -> HashMap<String, Reactor> {
	let mut f = File::open(path).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	return json::decode(&buffer[..]).unwrap();
}
