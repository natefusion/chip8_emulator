use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 20;

pub struct Chip8SDL {
    event_pump: sdl2::EventPump,
    canvas: Canvas<Window>,
}

impl Chip8SDL {
    pub fn initialize(game_name: &str) -> Chip8SDL {
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
        
        Chip8SDL { event_pump, canvas }
    }
    
    pub fn draw_frame(&mut self, gfx: &[u8]) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        for (i, val) in gfx.iter().enumerate() {
            if *val == 1 {
                let px = i as u32 % WIDTH;
                let py = (i as u32 - px) / WIDTH;

                let sx = (px * SCALE) as i32;
                let sy = (py * SCALE) as i32;
                self.canvas.fill_rect(Rect::new(sx, sy, SCALE, SCALE)).unwrap();
            }
        }

        self.canvas.present();
    }
    
    pub fn handle_events(&mut self, keys: &mut [u8]) -> usize {
        for event in self.event_pump.poll_iter() {
            let (key_state, keycode) = match event {
                Event::Quit { .. } => { return 1; },

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::L) => { return 2; },
                    Some(Keycode::J) => { return 3; },
                    Some(Keycode::K) => { return 4; },
                    _ => (1, keycode),
                },

                Event::KeyUp { keycode, .. } => (0, keycode),

                _ => { continue; },
            };

            if let Some(key) = keycode {
                keys[
                    match key {
                        Keycode::Num1 => 0x1, Keycode::Num2 => 0x2,
                        Keycode::Num3 => 0x3, Keycode::Num4 => 0xC,
                        Keycode::Q => 0x4,    Keycode::W => 0x5,
                        Keycode::E => 0x6,    Keycode::R => 0xD,
                        Keycode::A => 0x7,    Keycode::S => 0x8,
                        Keycode::D => 0x9,    Keycode::F => 0xE,
                        Keycode::Z => 0xA,    Keycode::X => 0x0,
                        Keycode::C => 0xB,    Keycode::V => 0xF,
                        _ => continue,
                    }
                ] = key_state;
            }
        }
        
        0
    }
}
