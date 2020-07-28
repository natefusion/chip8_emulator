extern crate sdl2;

use chip8::Chip8;
mod chip8;

fn setup_graphics() {}
fn setup_input() {}
fn draw_graphics() {}


fn main() {
	let my_chip8 = Chip8::initialize();

	// Set up render system and register input callbacks
	setup_graphics();
	setup_input();

	// Initialize the Chip8 system and load the game into memory
	let my_chip8 = Chip8::initialize();
	my_chip8.load_game("");

	// Emulation loop
	loop {
		// Emulate one cycle
		my_chip8.emulate_cycle();

		// If the draw flag is set, update the screen
		if my_chip8.draw_flag {
			draw_graphics();
		}

		// Store key press state (press and release)
		my_chip8.set_keys();
	}
}
