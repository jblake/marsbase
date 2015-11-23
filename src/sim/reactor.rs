use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Reactor {
	pub inputs: HashMap<String, f64>,	// Both these are maps from fluid to mole
	pub outputs: HashMap<String, f64>,
}

#[allow(dead_code)]
impl Reactor {
	pub fn new() -> Reactor { Reactor { inputs: HashMap::new(), outputs: HashMap::new() } }

	// The environment is a map of (fluidname, (amount, storagespace))
	// Both amount and storagespace are expressed in moles
	// There must be enough material to react, and some space for the result
	pub fn react(&self, env: &mut HashMap<String, (f64, f64)>, mut min_reactivity: f64, mut reactivity: f64) -> f64 {
		if min_reactivity < 0.0 {
			min_reactivity = 0.0;
		}
		for (key, need) in self.inputs.iter() {
			match env.get(key) {
				None => return 0.0,
				Some(&(avail, _)) => {
					let new_reactivity = avail / *need;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		for (key, make) in self.outputs.iter() {
			match env.get(key) {
				None => return 0.0,
				Some(&(avail, space)) => {
					let new_reactivity = (space - avail) / *make;
					if new_reactivity < reactivity {
						reactivity = new_reactivity;
					}
				}
			}
		}
		if reactivity < min_reactivity {
			return 0.0;
		}
		if reactivity.is_infinite() {
			panic!("Reaction has infinite reactivity");
		}
		for (key, need) in self.inputs.iter() {
			(*env.get_mut(key).unwrap()).0 -= reactivity * *need;
		}
		for (key, make) in self.outputs.iter() {
			(*env.get_mut(key).unwrap()).0 += reactivity * *make;
		}
		return reactivity;
	}
}

pub fn load_reactors(path: &str) -> HashMap<String, Reactor> {
	let mut f = File::open(path).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	return json::decode(&buffer[..]).unwrap();
}
