extern crate gtk;
extern crate rustc_serialize;

mod sim;

use gtk::signal::WidgetSignals;
use gtk::traits::container::ContainerTrait;
use gtk::traits::widget::WidgetTrait;
use gtk::traits::window::WindowTrait;

fn main() {
	gtk::init().unwrap();

	let fluids = sim::load_fluids("data/fluids.json");
	let solids = sim::load_solids("data/solids.json");
	let reactors = sim::load_reactors("data/reactors.json");

	let top = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
	top.connect_delete_event(|_, _| { gtk::main_quit(); return gtk::signal::Inhibit(false); });
	top.set_title("MarsBase");

	let lbl = gtk::Label::new("'Sup, nerds?").unwrap();
	top.add(&lbl);

	top.show_all();

	gtk::main();
}
