use rand::Rng;
pub struct Chip8 {
	/* Materials:
	 * http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
	 * https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
     * http://mattmik.com/files/chip8/mastering/chip8.html
	 *
	 * System memory map:
	 * 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
	 * 0x050-0x0A0 - Used for the builtin 4x5 pixel font set (0-F)
	 * 0x200-0xFFF - Program ROM and work RAM
	 */

	// Chip 8 has 35 opcodes, each 2 bytes long
	opcode: u16,
	// Chip 8 has 4K bytes of memory. 
	memory: [u8;4096],
	// Chip 8 has 15 8-bit general purpose registers (v0,v1..vE)
	// The 16th is used for the carry flag
	v: [u8;16],
	// Index register 'i' and program counter 'pc' with a range of 0x000 to 0xFFF
	i: u16,
	pc: u16,
	// For graphics. array of on (1) or off (0) pixels
	gfx: [u8;64*32],
	// Two timers that count at 60Hz, when set above 0 they count down.
	delay_timer: u8,
	sound_timer: u8,
	// Stack is used to remeber the current location 
	// before you jump or call a subroutine.
	// The stack pointer is used to remember which level of the stack is used
	stack: [u16;16],
	sp: u16,
	// Load hex based keypad (0x0-0xF)
	key: [u8;16],
		
	// Creates the fontset, there are 16 total characters
	fontset: [u8;80],

}

impl Chip8 {
	pub fn initialize() -> Chip8 {
		let mut chip = Chip8 {
			pc:     0x200,       // Program counter starts at 0x200
			opcode: 0,           // Reset current opcode
			i:      0,           // Reset index register
			sp:     0,           // Reset stack pointer

			gfx:    [0;64*32],   // Clear display
			stack:  [0;16],      // Clear stack
			v:      [0;16],      // Clear registers v0-vF
			memory: [0;4096],    // Clear memory
			
			delay_timer: 0,      // Reset delay timer
			sound_timer: 0,      // Reset sound timer
			key: [0;16],

			fontset: {           // Character values for fontset
				[0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
				0x20, 0x60, 0x20, 0x20, 0x70,  // 1
				0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
				0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
				0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
				0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
				0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
				0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
				0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
				0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
				0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
				0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
				0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
				0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
				0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
				0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
			},
		};

		// fontset is stored from 0x000 to 0x1FF in memory
		for a in 0..80 { chip.memory[a] = chip.fontset[a] }

		return chip;
	}

	pub fn emulate_cycle(&mut self) {
		// Fetch opcode
		// Merges two successive bytes from the memory to form an opcode
		self.opcode = self.memory[self.pc] << 8 | self.memory[self.pc+1];

		// Decode opcode
		// Reads first 4 bits of the opcode (highest 4 bits)
		match self.opcode & 0xF000 {
			0x0000 => {
				// Some opcodes need the last four bits to be read (lowest 4 bits)
				match self.opcode & 0x000F {
					// 00E0: Clears the screen
					0x0000 => for a in 0..self.gfx.iter_mut() { *a = 0; }

					// 00EE: Returns from the subroutine
					0x000E => {
						// I don't know where the 'top' of the stack is, so I'll assume it's at the end
						let top = self.stack[15];
						self.pc = top;
						self.sp -= 1;
					}
					_ => println("Unknown opcode [0x0000]: 0x{:x}",self.opcode);
				}
			}
			
			// 1NNN: Jump to location NNN
			0x1000 => self.pc = self.opcode & 0x0FFF;

			// 2NNN: Execute subroutine starting at NNN
			0x2000 => {
				self.stack[self.sp] = self.pc;
				sp += 1;
				self.pc = self.opcode & 0x0FFF;
			}
			
			// 3xkk: Skip next instruction if vx == kk
			0x3000 => {
				if self.v[(self.opcode & 0x0F00) >> 8] == self.opcode & 0x00FF {
					self.pc += 2;
				}
			}
			
			// 4xkk: Skip next instruction if vx != kk
			0x4000 => {
				if self.v[(self.opcode & 0x0F00) >> 8] != self.opcode & 0x00FF {
					self.pc += 2;
				}
			}
			
			// 5xy0: Skip next instruction if vx == vy
			0x5000 => {
				if self.v[(self.opcode & 0x0F00) >> 8] == self.v[self.opcode & 0x00F0) >> 4] {
					pc += 2;
				}
			}
			
			// 6xkk: Set vx = kk
			0x6000 => self.v[self.opcode & 0x0F00) >> 8] = self.opcode & 0x00FF;
			
			// 7xkk: Set vx = vx + kk
			0x7000 => {
				self.v[(self.opcode & 0x0F00) >> 8] =
					self.v[(self.opcode & 0x0F00) >> 8] + self.opcode & 0x00FF;
			}
			
			0x8000 => {
				match self.opcode & 0x000F {
					// 8xy0: Set vx = vy
					0x0000 => self.v[(self.opcode & 0x0F00) >> 8] = self.v[(self.opcode & 0x00F0) >> 4];
					
					// 8xy1: Set vx = vx | vy
					0x0001 => {
						self.v[(self.opcode & 0x0F00) >> 8] =
							self.v[(self.opcode & 0x0F00) >> 8] | self.v[(self.opcode & 0x00F0) >> 4];
					}
					
					// 8xy2: Set vx = vx & vy
					0x0002 => {
						self.v[(self.opcode & 0x0F00) >> 8] =
							self.v[(self.opcode & 0x0F00) >> 8] & self.v[(self.opcode & 0x00F0) >> 4];
					}
					
					// 8xy3: set vx = xv ^ xy
					0x0003 => {
						self.v[(self.opcode & 0x0F00) >> 8] =
							self.v[(self.opcode & 0x0F00) >> 8] ^ self.v[(self.opcode & 0x00F0) >> 4];
					}
					
					// 8xy4: Adds the value of register vy to register vx
					0x0004 => {
						if self.v[(self.opcode & 0x00F0) >> 4] > (0xFF - self.v[(self.opcode & 0x0F00) >> 8)] {
							self.v[0xF] = 1; // carry
						} else {
							self.v[0xF] = 0;
						}
						self.v[(self.opcode & 0x0F00) >> 8] += self.v[(self.opcode & 0x00F0) >> 4];
						pc += 2;
						
					// 8xy5: Set vx = vx - vy, set vf = NOT borrow
					0x0005 => {
						if self.v[(self.opcode & 0x0F00) >> 8] > self.v[(self.opcode & 0x00F0) >> 4] {
							self.v[0xF] = 1;
						} else {
							self.v[0xF] = 0;
						}
						self.v[(self.opcode & 0x0F00) >> 8] -= self.v[(self.opcode & 0x0F00) >> 8];
					}
					
					// 8xy6: Set vx = vx SHR 1
					0x0006 => {
						// If LSB is equal to 1
						if (self.v[(self.opcode & 0x0F00) >> 8] & 0x000F) >> 3 == 1 {
							self.v[0xF] = 1;
						} else {
							self.v[0xF] = 0;
						}
						self.v[(self.opcode & 0x0F00) >> 8] /= 2;
					}
					
					// 8xy7: Set vx = vy - vx, set vf = NOT borrow
					0x0007 => {
						if self.v[(self.opcode & 0x00F0) >> 4] > self.v[(self.opcode & 0x0F00) >> 8] {
							self.v[0xF] = 1;
						} else {
							self.v[0xF] = 0;
						}
						self.v[(self.opcode & 0x0F00) >> 8] =
							self.v[(self.opcode & 0x00F0) >> 4] - self.v[(self.opcode & 0x0F00) >> 8];
					}
					
					// 8xye: Set vx = vx SHL 1
					0x000e => {
						//if MSB is equal to 1
						if (self.v[(self.opcode & 0x0F00) >> 8] & 0xF000) >> 15 == 1 {
							self.v[0xF] = 1;
						} else {
							self.v[0xF] = 0;
						}
						self.v[(self.opcode & 0x0F00) >> 8] *= 2;
					}
					_ => println!("Unknown opcode [0x8000]: 0x{:x}",self.opcode);
				}
			}
			
			// Skip next instruction if vx != vy
			0x9000 => {
				if self.v[(self.opcode & 0x0F00) >> 8] != self.v[(self.opcode & 0x00F0) >> 4] {
					self.pc += 2;
				}
			}

			// ANNN: Store memory address NNN in register i
			0xA000 => {
				self.i = self.opcode & 0x0FFF;
				self.pc += 2;
			}
			
			// BNNN: Jump to location NNN + v0
			0xB000 => self.pc = (opcode & 0x0FFF) + v[0];
			
			// cxkk: Set vx = random byte AND kk
			0xC000 => {
				self.v[(self.opcode & 0x0F00) >> 8] = 
					rand::thread_rng().gen_range(0,255) & self.opcode & 0x00FF;
			}
			
			// dxyn: Display n-byte sprite starting at memory location 'i' at (vx, vy), set vf = collision
            // Looks hard
			0xD000 => {}

            0xE000 => {
                match self.opcode & 0x00FF {
                    // ex9e: Skip next instruction if key with the value of vx is pressed
                    // Double check
                    0x009E => {
                        if self.key[self.v[(self.opcode 7 0x0F00) >> 8]] == 1 {
                            self.pc += 2;
                        }
                    }

                    // exa1: Skip next instruction if key with the value of vx is not pressed
                    // Double check
                    0x00A1 => {
                        if self.key[self.v[(self.opcode & 0x0F00) >> 8]] == 0 {
                            self.pc += 2;
                        }
                    }
                }
            }
			
			0xF000 => {
				match opcode & 0x00FF {
                    // fx07: Set vx = delay timer value
                    0x0007 => self.v[(self.opcode & 0x0F00) >> 8] = self.delay_timer;

                    // fx0a: Wait for a key press, store the value of the key in vx
                    // Looks hard
                    0x000A => {}

                    // fx15: Set delay timer = vx
                    0x0015 => self.delay_timer = self.v[(self.opcode & 0x0F00) >> 8];

                    // fx18: Set sound timer = vx
                    0x0018 => self.sound_timer = self.v[(self.opcode & 0x0F00) >> 8];

                    // fx1e: Set i = i + vx
                    0x001E => self.i += self.v[(self.opcode & 0x0F00) >> 8];

                    // fx29: Set i = location of sprite for digit vx
                    // This looks hard, I'll do it later
                    0x0029 => {}

					// fx33: Store the binary-coded decimal equivalent of the value
					// stored in register vx at address i, i+1, and i+2
					0x0033 => {
						self.memory[self.i]   = self.v[(self.opcode & 0x0F00) >> 8] / 100;
						self.memory[self.i+1] = (self.v[(self.opcode & 0x0F00) >> 8] / 10) % 10;
						self.memory[self.i+2] = (self.v[(self.opcode & 0x0F00) >> 8] % 100) % 10;
						pc += 2;
					}

                    // fx55: Store registers v0 through vx in memory starting at location i
                    // Looks hard
                    0x0055 => {}

                    // fx65: Read registers V0 through Vx from memory starting at location i
                    // Looks hard
                    0x0065 => {}
					_ => println!("Unknown opcode: 0x{:x}",self.opcode);
				}
			}
		}

		// Update timers
		if self.delay_timer > 0 {
			delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			if sound_timer == 1 { println!("BEEP!"); }
			sound_timer -= 1;
		}
	}

	pub fn load_game(&mut self) {}
	pub fn draw_flag(&mut self) {}
	pub fn set_keys(&mut self) {}
}
