use std::{time::Duration,thread::sleep,process,env};
mod chip8sdl;
mod chip8;

fn main() {
    let game = match env::args().nth(1) {   
        Some(game) => game,
        _=> {
            eprintln!("Error, please enter a game file as an argument");
            process::exit(1);
        },
    };

    let mut my_chip8 = chip8::Chip8::initialize();
    let mut my_chip8sdl = chip8sdl::Chip8SDL::initialize(&game);

    my_chip8.load_game(&game);
    my_chip8.sound_state = false;

    // Emulation loop
    loop {
        my_chip8.emulate_cycle();
        if my_chip8.draw_flag {
            my_chip8sdl.draw_frame(&my_chip8);
        }
        if my_chip8sdl.handle_events(&mut my_chip8) { break; }
        sleep(Duration::from_micros(7000));
    }
}
