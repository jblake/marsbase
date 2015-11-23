extern crate rustc_serialize;

mod sim;

use std::collections::HashMap;
use std::f64;

fn main() {
	let fluids = sim::load_fluids("data/fluids.json");
	let reactors = sim::load_reactors("data/reactors.json");

	let ref reactor = reactors["sabatier"];

	let mut env: HashMap<String, (f64, f64)> = HashMap::new();

	env.insert("carbon dioxide".to_owned(), (1.0, 2.0));
	env.insert("hydrogen".to_owned(), (4.0, 8.0));
	env.insert("methane".to_owned(), (0.0, 2.0));
	env.insert("water".to_owned(), (0.0, 4.0));

	println!("Starting environment:");
	for (key, &(moles, _)) in env.iter() {
		println!("\t{} grams of {}", fluids[key].grams_from_moles(moles), key);
	
	}

	let reactivity = reactor.react(&mut env, 0.0, f64::INFINITY);

	if reactivity >= 0.0 {
		println!("Reaction took place! (reactivity was {})", reactivity);
	} else {
		println!("No reaction took place!");
	}

	println!("After reaction:");
	for (key, &(moles, _)) in env.iter() {
		println!("\t{} grams of {}", fluids[key].grams_from_moles(moles), key);
	}
}
