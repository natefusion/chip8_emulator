use std::time::Duration;
use chip8::Chip8;
//use gfx::Gfx;
//mod gfx;
mod chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;


const GAME: &str = "/home/nathan/Downloads/c8games/TETRIS";

fn draw_frame(canvas: &mut Canvas<Window>, my_chip8: &Chip8)
{
	canvas.set_draw_color(Color::RGB(255,255,255));
	canvas.clear();
	canvas.set_draw_color(Color::RGB(0,0,0));

	let mut x: i32 = 0;
	let mut y: i32 = 0;
	for (i, val) in my_chip8.gfx.iter().enumerate()
	{
		if i > 0 && i % 64 == 0 { y += 10; x = 0; }
		if i >= my_chip8.gfx.len()-1 { y = 0; }
		if *val == 1 { canvas.fill_rect(Rect::new(x,y,10,10)).unwrap(); }
		x += 10;
	}
	canvas.present();
}

fn handle_events(event_pump: &mut sdl2::EventPump, my_chip8: &mut Chip8) -> bool
{
	let mut exit = false;
	for event in event_pump.poll_iter()
	{
		match event
		{
			Event::Quit {..} |
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => exit = true,

			// Maybe change key[] to a bool?
			Event::KeyDown { keycode: Some(Keycode::Num0), .. } => my_chip8.key[0]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num1), .. } => my_chip8.key[1]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num2), .. } => my_chip8.key[2]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num3), .. } => my_chip8.key[3]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num4), .. } => my_chip8.key[4]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num5), .. } => my_chip8.key[5]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num6), .. } => my_chip8.key[6]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num7), .. } => my_chip8.key[7]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num8), .. } => my_chip8.key[8]  = 1,
			Event::KeyDown { keycode: Some(Keycode::Num9), .. } => my_chip8.key[9]  = 1,
			Event::KeyDown { keycode: Some(Keycode::A), .. }    => my_chip8.key[10] = 1,
			Event::KeyDown { keycode: Some(Keycode::B), .. }    => my_chip8.key[11] = 1,
			Event::KeyDown { keycode: Some(Keycode::C), .. }    => my_chip8.key[12] = 1,
			Event::KeyDown { keycode: Some(Keycode::D), .. }    => my_chip8.key[13] = 1,
			Event::KeyDown { keycode: Some(Keycode::E), .. }    => my_chip8.key[14] = 1,
			Event::KeyDown { keycode: Some(Keycode::F), .. }    => my_chip8.key[15] = 1,
				
			Event::KeyUp { keycode: Some(Keycode::Num0), .. } => my_chip8.key[0]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num1), .. } => my_chip8.key[1]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num2), .. } => my_chip8.key[2]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num3), .. } => my_chip8.key[3]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num4), .. } => my_chip8.key[4]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num5), .. } => my_chip8.key[5]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num6), .. } => my_chip8.key[6]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num7), .. } => my_chip8.key[7]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num8), .. } => my_chip8.key[8]  = 0,
			Event::KeyUp { keycode: Some(Keycode::Num9), .. } => my_chip8.key[9]  = 0,
			Event::KeyUp { keycode: Some(Keycode::A), .. }    => my_chip8.key[10] = 0,
			Event::KeyUp { keycode: Some(Keycode::B), .. }    => my_chip8.key[11] = 0,
			Event::KeyUp { keycode: Some(Keycode::C), .. }    => my_chip8.key[12] = 0,
			Event::KeyUp { keycode: Some(Keycode::D), .. }    => my_chip8.key[13] = 0,
			Event::KeyUp { keycode: Some(Keycode::E), .. }    => my_chip8.key[14] = 0,
			Event::KeyUp { keycode: Some(Keycode::F), .. }    => my_chip8.key[15] = 0,
			_ => {}
		}
	}
	return exit;
}

fn main()
{
	let mut my_chip8 = Chip8::initialize();

	// Set up render system and register input callbacks
	let sdl_context = sdl2::init().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();

	// I know it's long, deal with it :^)
	let mut canvas = sdl_context
				.video()
				.unwrap()
				.window("Spooky", 640, 320)
				.position_centered()
				.build()
				.unwrap()
				.into_canvas()
				.build()
				.unwrap();

	my_chip8.load_game(GAME);

	// Emulation loop
	loop
	{
		my_chip8.emulate_cycle();
		if handle_events(&mut event_pump, &mut my_chip8) { break; }
		if my_chip8.draw_flag { draw_frame(&mut canvas, &my_chip8); }
		std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
	}
}
