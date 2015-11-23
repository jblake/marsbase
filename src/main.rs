extern crate rustc_serialize;

use std::f64;

mod sim;

fn main() {
	let fluids = sim::load_fluids("data/fluids.json");
	let solids = sim::load_solids("data/solids.json");
	let reactors = sim::load_reactors("data/reactors.json");

	let mut r = sim::ReactorContext::new(&reactors["rehydrate"], f64::INFINITY, &fluids, &solids);

	println!("{:?}\n", r);

	*r.fluid_avail.get_mut("water").unwrap() = 1000000.0;
	*r.solid_avail.get_mut("dehydrated food").unwrap() = 100;

	println!("{:?}\n", r);

	println!("reactivity {}\n", r.react());

	println!("{:?}", r);
}
