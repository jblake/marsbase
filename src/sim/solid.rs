use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Solid {
	pub mass: f64,	// grams
}

#[allow(dead_code)]
impl Solid {
}

pub fn load_solids(path: &str) -> HashMap<String, Solid> {
	let mut f = File::open(path).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	return json::decode(&buffer[..]).unwrap();
}
