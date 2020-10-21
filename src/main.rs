use std::time::Duration;
mod chip8sdl;
mod chip8;

fn main() {
    let game = std::env::args().nth(1).expect("Please enter a game file as an argument");

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

    //Remove this and instead count delay timer down at 60hz in the future
    //let duration = duration::from_millis(16.67);
    std::thread::sleep(Duration::from_micros(7000));
    }
}
