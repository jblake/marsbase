use rustc_serialize::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use super::unit::Unit;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Fluid {
	pub density: f64,	// grams per litre
	pub mol_mass: f64,	// grams per mol
}

#[allow(dead_code)]
impl Fluid {
	pub fn grams_from_litres(&self, litres: f64) -> f64 { litres * self.density }
	pub fn litres_from_grams(&self, grams: f64)  -> f64 { grams / self.density }
	pub fn grams_from_mols  (&self, mols: f64)   -> f64 { mols * self.mol_mass }
	pub fn mols_from_grams  (&self, grams: f64)  -> f64 { grams / self.mol_mass }
	pub fn litres_from_mols (&self, mols: f64)   -> f64 { self.litres_from_grams(self.grams_from_mols(mols)) }
	pub fn mols_from_litres (&self, litres: f64) -> f64 { self.mols_from_grams(self.grams_from_litres(litres)) }

	pub fn grams_from(&self, val: f64, unit: Unit) -> f64 {
		match unit {
			Unit::gram => val,
			Unit::litre => self.grams_from_litres(val),
			Unit::mol => self.grams_from_mols(val)
		}
	}

	pub fn litres_from(&self, val: f64, unit: Unit) -> f64 {
		match unit {
			Unit::gram => self.litres_from_grams(val),
			Unit::litre => val,
			Unit::mol => self.litres_from_mols(val)
		}
	}

	pub fn mols_from(&self, val: f64, unit: Unit) -> f64 {
		match unit {
			Unit::gram => self.mols_from_grams(val),
			Unit::litre => self.mols_from_litres(val),
			Unit::mol => val
		}
	}
}

pub fn load_fluids(path: &str) -> HashMap<String, Fluid> {
	let mut f = File::open(path).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	json::decode(&buffer[..]).unwrap()
}
