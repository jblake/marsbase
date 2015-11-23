extern crate rustc_serialize;

mod sim;

fn main() {
	let fluids = sim::load_fluids("data/fluids.json");
	let solids = sim::load_solids("data/solids.json");
	let reactors = sim::load_reactors("data/reactors.json");

	let mut r = sim::ReactorContext::new(&reactors["rehydrate"], 1000000.0, &fluids, &solids);

	println!("{:?}\n", r);

	*r.fluid_avail.get_mut("water").unwrap() = r.fluid_space["water"];
	*r.fluid_space.get_mut("water").unwrap() = 0.0;

	*r.solid_avail.get_mut("dehydrated food").unwrap() = r.solid_space["dehydrated food"];
	*r.solid_space.get_mut("dehydrated food").unwrap() = 0;

	println!("{:?}\n", r);

	println!("reactivity {}\n", r.react());

	println!("{:?}", r);
}
