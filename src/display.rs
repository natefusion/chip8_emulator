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

    fn set_key(key: Keycode, pressed: u8, my_chip8: &mut Chip8) {
	let i = match key {
	    Keycode::Num1 => 0x1,
            Keycode::Num2 => 0x2,
            Keycode::Num3 => 0x3,
            Keycode::Num4 => 0xC,
            Keycode::Q => 0x4,
            Keycode::W => 0x5,
            Keycode::E => 0x6,
            Keycode::R => 0xD,
            Keycode::A => 0x7,
            Keycode::S => 0x8,
            Keycode::D => 0x9,
            Keycode::F => 0xE,
            Keycode::Z => 0xA,
            Keycode::X => 0x0,
            Keycode::C => 0xB,
            Keycode::V => 0xF,
	    _=> return,
	};
	my_chip8.key[i] = pressed;
    }
    
    pub fn handle_events(&mut self, my_chip8: &mut Chip8) -> bool {
	let mut exit = false;
	for event in self.event_pump.poll_iter() {
	    match event {
		Event::Quit { .. } => exit = true,

		// Maybe do some refactoring of this code
		Event::KeyDown { keycode, .. } => {
		    if let Some(key) = keycode {
			Display::set_key(key, 1, my_chip8);
		    }
		}

		Event::KeyUp { keycode, .. } => {
		    if let Some(key) = keycode {
			Display::set_key(key, 0, my_chip8);
		    }
		}
		
		_ => ()
	    }
	}
	exit
    }
}
