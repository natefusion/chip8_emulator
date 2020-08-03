extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Gfx
{
	event_pump: sdl2::EventPump,
	canvas: Canvas<Window>,
}

impl Gfx
{
	fn initialize(title: &str) -> Gfx
	{
		let sdl_context = sdl2::init().unwrap();
		Gfx
		{
			// Need this separate because of event handling (see sdl_test)
			event_pump: sdl_context.event_pump().unwrap(),
			
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
	
	fn draw_frame(&mut self, my_chip8: &Chip8)
	{
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		for (i, val) in my_chip8.gfx.iter().enumerate()
		{
			if i > 0 && i % 64 == 0 { y += 10; x = 0; }

			if *val == 1 { self.canvas.fill_rect(Rect::new(x,y,10,10)).unwrap(); }
			x += 10;
		}

	fn handle_events(&mut self, my_chip8: &Chip8) -> return bool
	{
		let mut exit = false;
		for event in self.event_pump.poll_iter()
		{
			match event
			{
				Event::Quit {..} |
				Event::Keydown { keycode: Some(Keycode::Escape), .. } => exit = true,
				_ => {}
			}
		}
		return exit;
	}
}
