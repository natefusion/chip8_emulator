mod chip8sdl;
mod chip8;

use std::{process, env, fs::File};

fn main() {
    let (mut file, filename) = match env::args().nth(1) {   
        Some(filename) => (File::open(&filename).unwrap(), filename),
        None => {
            eprintln!("Error, please enter a game file as an argument");
            process::exit(1);
        },
    };

    let mut my_chip8 = chip8::Chip8::initialize();

    // maybe use rust_minifb instead
    let mut my_chip8sdl = chip8sdl::Chip8SDL::initialize(&filename);

    my_chip8.load_game(&mut file);
    my_chip8.sound_state = false;

    // Emulation loop
    loop {
        my_chip8.emulate_cycle();
        
        if my_chip8.draw_flag {
            my_chip8sdl.draw_frame(&my_chip8.gfx);
        }
        
        match my_chip8sdl.handle_events(&mut my_chip8.keys) {
            1 => break, // quits game
            2 => my_chip8.load_game(&mut file), // reloads game
            _ => {},
        }
    }
}
