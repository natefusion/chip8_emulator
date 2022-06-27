mod chip8sdl;
mod chip8;

use std::{process, env, fs::File, time::Instant, time::Duration};

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

    let frame_time = Duration::from_micros(16666);
    let mut last = Instant::now();
    let mut origin = last + frame_time / 2;
    let tickrate = 20;
    // Emulation loop
    loop {
        let diff = Instant::now() - last;
        last += diff;

        for _ in 0..2 {
            if origin >= last - frame_time { break; }
            
            for _ in 0..tickrate {
                if my_chip8.waiting { break; }
                
                my_chip8.emulate_cycle();
            }

            origin += frame_time;
        }

        if my_chip8.draw_flag {
            my_chip8sdl.draw_frame(&my_chip8.gfx);
            my_chip8.draw_flag = false;
        }

        if my_chip8.st > 0 { my_chip8.st -= 1; }
        if my_chip8.dt > 0 { my_chip8.dt -= 1; }
        
        match my_chip8sdl.handle_events(&mut my_chip8.keys) {
            1 => break, // quits game
            2 => my_chip8.load_game(&mut file), // reloads game // L
            _ => {},
        }

       std::thread::sleep(frame_time); 
    }
}
