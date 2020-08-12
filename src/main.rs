use std::time::Duration;
use chip8::Chip8;
mod chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;


const GAME_DIR: &str = "/home/nathan/Downloads/c8games/";
const GAME_NAME: &str = "TETRIS";
const MOD: u32 = 10;

fn draw_frame(canvas: &mut Canvas<Window>, my_chip8: &Chip8)
{
	canvas.set_draw_color(Color::RGB(255,255,255));
	canvas.clear();
	canvas.set_draw_color(Color::RGB(0,0,0));

	for (y, i) in my_chip8.gfx.iter().enumerate()
	{
		for (x, val) in i.iter().enumerate()
		{
			let x = (x as u32 * MOD) as i32;
			let y = (y as u32 * MOD) as i32;
			if *val == 1 { canvas.fill_rect(Rect::new(x,y,MOD,MOD)).unwrap(); }
		}
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
		.window(GAME_NAME, 64*MOD, 32*MOD)
		.position_centered()
		.build()
		.unwrap()
		.into_canvas()
		.build()
		.unwrap();

	my_chip8.load_game(GAME_DIR, GAME_NAME);

	// Emulation loop
	loop
	{
		draw_frame(&mut canvas, &my_chip8);
		my_chip8.emulate_cycle();
		if handle_events(&mut event_pump, &mut my_chip8) { break; }
		std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
	}
}
