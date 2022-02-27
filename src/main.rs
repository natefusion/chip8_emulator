mod chip8sdl;
mod chip8;

use std::{process, env, fs::File, time::Instant};

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

    let mut delay = 1; // milliseconds
    let mut last = Instant::now();
    // Emulation loop
    loop {
        let current = Instant::now();
        let elapsed = current - last;
        
        let dt = elapsed.as_millis();

        if dt > delay {
            last = current;
            my_chip8.emulate_cycle();

            if my_chip8.draw_flag {
                my_chip8sdl.draw_frame(&my_chip8.gfx);
                my_chip8.draw_flag = false;
            }
        }

        let change_delay = |sub, d: &mut u128| { if *d > 0 && sub { *d -= 1; } else if !sub { *d += 1; } println!("{}", d); };
        
        match my_chip8sdl.handle_events(&mut my_chip8.keys) {
            1 => break, // quits game
            2 => my_chip8.load_game(&mut file), // reloads game // L
            3 => { change_delay(true, &mut delay); }, // J
            4 => { change_delay(false, &mut delay); }, // K
            _ => {},
        }
    }
}
