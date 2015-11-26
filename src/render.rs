extern crate cairo;

use latch::Latch;

#[derive(Clone, Debug)]
pub enum State {
	Loading {
		message: String,
	}
}

unsafe impl Send for State { }

pub fn render(latch: &mut Latch<State>, ctx: cairo::Context) {
	let st = latch.get();

	match *st {

		State::Loading { ref message } => {
			ctx.set_source_rgb(0.0, 0.0, 0.0);
			ctx.paint();

			ctx.move_to(10.0, 34.0);
			ctx.select_font_face("sans", cairo::enums::FontSlant::Normal, cairo::enums::FontWeight::Bold);
			ctx.set_font_size(24.0);
			ctx.set_source_rgb(1.0, 1.0, 1.0);
			ctx.show_text(message);
		}

	}
}
