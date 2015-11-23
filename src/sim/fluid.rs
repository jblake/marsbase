use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Fluid {
	pub density: f64,	// grams per litre
	pub mole_mass: f64,	// grams per mole
}

#[allow(dead_code)]
impl Fluid {
	pub fn new(density: f64, mole_mass: f64) -> Fluid { Fluid { density: density, mole_mass: mole_mass } }

	pub fn grams_from_litres(&self, litres: f64) -> f64 { litres * self.density }
	pub fn litres_from_grams(&self, grams: f64)  -> f64 { grams / self.density }
	pub fn grams_from_moles (&self, moles: f64)  -> f64 { moles * self.mole_mass }
	pub fn moles_from_grams (&self, grams: f64)  -> f64 { grams / self.mole_mass }
	pub fn litres_from_moles(&self, moles: f64)  -> f64 { self.litres_from_grams(self.grams_from_moles(moles)) }
	pub fn moles_from_litres(&self, litres: f64) -> f64 { self.moles_from_grams(self.grams_from_litres(litres)) }
}

pub fn load_fluids(path: &str) -> HashMap<String, Fluid> {
	let mut f = File::open(path).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	return json::decode(&buffer[..]).unwrap();
}
