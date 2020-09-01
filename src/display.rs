use crate::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

pub struct Display {
    event_pump: sdl2::EventPump,
    canvas: Canvas<Window>,
}

impl Display {
    pub fn initialize(game_name: &str) -> Display {
	let sdl_context = sdl2::init().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();
	let canvas = sdl_context
	    .video()
	    .unwrap()
	    .window(game_name, WIDTH * SCALE, HEIGHT * SCALE)
	    .position_centered()
	    .build()
	    .unwrap()
	    .into_canvas()
	    .build()
	    .unwrap();
	
	Display {
	    event_pump,
	    canvas,
	}
    }
    
    pub fn draw_frame(&mut self, my_chip8: &Chip8) {
	self.canvas.set_draw_color(Color::RGB(255, 255, 255));
	self.canvas.clear();
	self.canvas.set_draw_color(Color::RGB(0, 0, 0));
	
	for (y, i) in my_chip8.gfx.iter().enumerate() {
	    for (x, val) in i.iter().enumerate() {
		let x = (x as u32 * SCALE) as i32;
		let y = (y as u32 * SCALE) as i32;
		if *val == 1 {
		    self.canvas.fill_rect(Rect::new(x, y, SCALE, SCALE)).unwrap();
		}
	    }
	}
	self.canvas.present();
    }
    
    pub fn handle_events(&mut self, my_chip8: &mut Chip8) -> bool {
	let mut exit = false;
	for event in self.event_pump.poll_iter() {
	    match event {
		Event::Quit {..}
		| Event::KeyDown { keycode: Some(Keycode::Escape),..} => exit = true,
		
		Event::KeyDown { keycode: Some(Keycode::Num1),..} => my_chip8.key[0]  = true,
		Event::KeyDown { keycode: Some(Keycode::Num2),..} => my_chip8.key[1]  = true,
		Event::KeyDown { keycode: Some(Keycode::Num3),..} => my_chip8.key[2]  = true,
		Event::KeyDown { keycode: Some(Keycode::Num4),..} => my_chip8.key[3]  = true,
		Event::KeyDown { keycode: Some(Keycode::Q),..}    => my_chip8.key[4]  = true,
		Event::KeyDown { keycode: Some(Keycode::W),..}    => my_chip8.key[5]  = true,
		Event::KeyDown { keycode: Some(Keycode::E),..}    => my_chip8.key[6]  = true,
		Event::KeyDown { keycode: Some(Keycode::R),..}    => my_chip8.key[7]  = true,
		Event::KeyDown { keycode: Some(Keycode::A),..}    => my_chip8.key[8]  = true,
		Event::KeyDown { keycode: Some(Keycode::S),..}    => my_chip8.key[9]  = true,
		Event::KeyDown { keycode: Some(Keycode::D),..}    => my_chip8.key[10] = true,
		Event::KeyDown { keycode: Some(Keycode::F),..}    => my_chip8.key[11] = true,
		Event::KeyDown { keycode: Some(Keycode::Z),..}    => my_chip8.key[12] = true,
		Event::KeyDown { keycode: Some(Keycode::X),..}    => my_chip8.key[13] = true,
		Event::KeyDown { keycode: Some(Keycode::C),..}    => my_chip8.key[14] = true,
		Event::KeyDown { keycode: Some(Keycode::V),..}    => my_chip8.key[15] = true,
		
		Event::KeyUp { keycode: Some(Keycode::Num1),..} => my_chip8.key[0]  = false,
		Event::KeyUp { keycode: Some(Keycode::Num2),..} => my_chip8.key[1]  = false,
		Event::KeyUp { keycode: Some(Keycode::Num3),..} => my_chip8.key[2]  = false,
		Event::KeyUp { keycode: Some(Keycode::Num4),..} => my_chip8.key[3]  = false,
		Event::KeyUp { keycode: Some(Keycode::Q),..}    => my_chip8.key[4]  = false,
		Event::KeyUp { keycode: Some(Keycode::W),..}    => my_chip8.key[5]  = false,
		Event::KeyUp { keycode: Some(Keycode::E),..}    => my_chip8.key[6]  = false,
		Event::KeyUp { keycode: Some(Keycode::R),..}    => my_chip8.key[7]  = false,
		Event::KeyUp { keycode: Some(Keycode::A),..}    => my_chip8.key[8]  = false,
		Event::KeyUp { keycode: Some(Keycode::S),..}    => my_chip8.key[9]  = false,
		Event::KeyUp { keycode: Some(Keycode::D),..}    => my_chip8.key[10] = false,
		Event::KeyUp { keycode: Some(Keycode::F),..}    => my_chip8.key[11] = false,
		Event::KeyUp { keycode: Some(Keycode::Z),..}    => my_chip8.key[12] = false,
		Event::KeyUp { keycode: Some(Keycode::X),..}    => my_chip8.key[13] = false,
		Event::KeyUp { keycode: Some(Keycode::C),..}    => my_chip8.key[14] = false,
		Event::KeyUp { keycode: Some(Keycode::V),..}    => my_chip8.key[15] = false,
		_ => {}
	    }
	}
	exit
    }
}
