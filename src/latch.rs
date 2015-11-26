use std::sync::mpsc;

pub struct Latch<T> {
	backend: mpsc::Receiver<T>,
	current: T,
}

unsafe impl<T: Send> Send for Latch<T> { }

impl<T> Latch<T> {
	pub fn new(r: mpsc::Receiver<T>) -> Latch<T> {
		let x = r.recv().unwrap();
		Latch {
			backend: r,
			current: x,
		}
	}

	pub fn get(&mut self) -> &T {
		loop {
			let x = self.backend.try_recv();
			match x {
				Ok(v) => self.current = v,
				Err(_) => break
			}
		}
		&self.current
	}
}
