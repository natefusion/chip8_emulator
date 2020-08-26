use std::time::Duration;
mod display;
mod chip8;

const GAME_DIR: &str = "/home/nathan/Downloads/c8games/";
const GAME_NAME: &str = "BRIX";

fn main() {
    let mut my_chip8 = chip8::Chip8::initialize();
    let mut my_display = display::Display::initialize(GAME_NAME);

    my_chip8.load_game(GAME_DIR, GAME_NAME);
    my_chip8.sound_state = false;

    // Emulation loop
    loop {
        my_chip8.emulate_cycle();
        if my_chip8.draw_flag {
	    my_display.draw_frame(&my_chip8);
	}
        if my_display.handle_events(&mut my_chip8) { break; }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }
}
