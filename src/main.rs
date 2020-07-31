use chip8::Chip8;
mod chip8;

const GAME: &'static str = "TETRIS"; // Games should be in ~/Downloads/c8games

fn main() {
	let my_chip8 = Chip8::initialize();

	// Set up render system and register input callbacks
	let screen = Gfx::initialize(GAME);

	// Initialize the Chip8 system and load the game into memory
	let my_chip8 = Chip8::initialize();
	my_chip8.load_game(GAME);

	// Emulation loop
	loop {
		// Emulate one cycle
		my_chip8.emulate_cycle();
		
		// If the draw flag is set, update the screen
		if my_chip8.draw_flag { screen.draw_frame(my_chip8); }
	}
}
