extern crate glib;
extern crate gtk;
extern crate rustc_serialize;

mod latch;
mod render;
mod sim;

use gtk::signal::Inhibit;
use gtk::signal::WidgetSignals;
use gtk::traits::FFIWidget;
use gtk::traits::container::ContainerTrait;
use gtk::traits::widget::WidgetTrait;
use gtk::traits::window::WindowTrait;
use std::sync::atomic;
use std::sync::mpsc;
use std::thread;

use latch::Latch;
use render::render;

fn main() {
	gtk::init().unwrap();

	let (send, recv) = mpsc::channel();
	send.send(render::State::Loading { message: "Loading...".to_string() });
	let mut latch = Latch::new(recv);

	let top = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
	top.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
	top.set_size_request(800, 600);
	top.set_title("MarsBase");

	let draw = gtk::DrawingArea::new().unwrap();
	unsafe {
		//
		// Why this unsafe block?
		//
		// As far as rust is concerned, this signal handler might live
		// forever, so it can't capture the reference to latch.
		// However, we know that draw is going to be destroyed before
		// latch is destroyed, via top.destroy after our main loop. So,
		// we use an unsafe pointer here to bypass the borrow checker
		// and allow our redraw handler to assume that we are still in
		// a region where latch is valid.
		//
		let p = &mut latch as *mut Latch<render::State>;
		draw.connect_draw(move |_, ctx| { render(&mut *p, ctx); Inhibit(true) });
	}
	top.add(&draw);

	//
	// Why this use of wrap/unwrap?
	//
	// Again, we have ownership issues with a DrawingArea. In this case, we
	// want to be able to copy it between closures, but it doesn't
	// implement the Copy trait. We just wrap and unwrap it via the
	// underlying pointer, and copy that. We will only use inside the
	// gameloop, which terminates before top is destroyed, so this is safe.
	// The use of AtomicPtr is because we need the Send trait on the
	// pointer that we're shipping, because the game loop is a separate
	// thread.
	//
	let raw_draw = atomic::AtomicPtr::new(draw.unwrap_widget());
	let redraw = move || {
		let d: gtk::DrawingArea = FFIWidget::wrap_widget(raw_draw.load(atomic::Ordering::Relaxed));
		glib::idle_add(move || { d.queue_draw(); glib::Continue(false) });
	};

	top.show_all();

	thread::spawn(move || gameloop(send, redraw));

	gtk::main();

	top.destroy();
}

fn gameloop<F: Fn()>(send: mpsc::Sender<render::State>, redraw: F) {
	thread::sleep_ms(1000);

	send.send(render::State::Loading { message: "Loading fluids...".to_string() });
	redraw();
	let fluids = sim::load_fluids("data/fluids.json");

	thread::sleep_ms(1000);

	send.send(render::State::Loading { message: "Loading solids...".to_string() });
	redraw();
	let solids = sim::load_solids("data/solids.json");

	thread::sleep_ms(1000);

	send.send(render::State::Loading { message: "Loading reactors...".to_string() });
	redraw();
	let reactors = sim::load_reactors("data/reactors.json");

	thread::sleep_ms(1000);

	send.send(render::State::Loading { message: "Just wasting time...".to_string() });
	redraw();

	thread::sleep_ms(1000);

	gtk::main_quit();
}
