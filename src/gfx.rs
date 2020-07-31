extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct Gfx {
	sdl_context: Result,
	canvas: Result,
}

impl Gfx {
	fn initialize(title: &'static str) -> Gfx {
		Gfx {
			// Need this separate because of event handling (see sdl_test)
			sdl_context: sdl2::init().unwrap(),
			
			// Creates the canvas for drawing things
			// I know it's long, deal with it :^)
			canvas: sdl_context
				.video()
				.unwrap()
				.window(title, 64*10, 32*10)
				.position_centered()
				.build()
				.unwrap()
				.into_canvas()
				.present_vsync()
				.build()
				.unwrap()
		}
	}
	
	fn draw_frame(&mut self, my_chip8: Chip8) {
		// Program data is stored starting at memory location 0x200 and ending at 0xFFF
		for a in 0x200..0xFFF {
			//convert memory to rectangles and put them in the right places
		}
	}
}
