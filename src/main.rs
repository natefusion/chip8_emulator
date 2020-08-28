use std::time::Duration;
mod display;
mod chip8;

fn main() {
    let game = std::env::args().nth(1).expect("Please enter a game file as an argument");

    let mut my_chip8 = chip8::Chip8::initialize();
    let mut my_display = display::Display::initialize(&game);

    my_chip8.load_game(&game);
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
