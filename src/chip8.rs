#![allow(non_camel_case_types, non_snake_case)]
use rand::Rng;
use std::{fs,process};

/* Materials:
 * http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
 * http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
 *
 * System memory map:
 * 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
 * 0x050-0x0A0 - Used for the builtin 4x5 pixel font set (0-F)
 * 0x200-0xFFF - Program ROM and work RAM
 */

pub struct Chip8 {
    // Function pointer arrays for opcodes
    t_main: [fn(&mut Chip8); 16],
    t_0000: [fn(&mut Chip8); 15],
    t_8000: [fn(&mut Chip8); 15],
    t_E000: [fn(&mut Chip8); 4],
    t_F000: [fn(&mut Chip8); 9],

    opcode: u16,
    memory: [u8; 4096],

    // Cpu registers, v[0xF] for 'carry flag'
    v: [u8; 16],

    // Index register 
    i: u16,
    
    // Program counter
    pc: u16,
    
    delay_timer: u8,
    sound_timer: u8,
      
    stack: [u16; 16],
    sp: u16,
    
    fontset: [u8; 80],
    
    pub gfx: [u8; 2048], // 64 x 32
    // Holds keyboard state
    pub key: [u8; 16],
    pub draw_flag: bool,
    pub sound_state: bool,

    // aliases for commonly used bits
    nnn: u16,
    nn:  u8,
    n:   usize,
    x:   usize,
    y:   usize,
}

impl Chip8 {
    pub fn initialize() -> Chip8 {
        let mut chip = Chip8 {
            t_main: {[
                    Chip8::i_0000, Chip8::i_1NNN, Chip8::i_2NNN, Chip8::i_3XNN,
                    Chip8::i_4XNN, Chip8::i_5XY0, Chip8::i_6XNN, Chip8::i_7XNN,
                    Chip8::i_8000, Chip8::i_9XY0, Chip8::i_ANNN, Chip8::i_BNNN,
                    Chip8::i_CXNN, Chip8::i_DXYN, Chip8::i_E000, Chip8::i_F000,
            ]},

            t_0000: {[
                    Chip8::i_00E0, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_00EE,
            ]},

            t_8000: {[
                    Chip8::i_8XY0, Chip8::i_8XY1, Chip8::i_8XY2,
                    Chip8::i_8XY3, Chip8::i_8XY4, Chip8::i_8XY5,
                    Chip8::i_8XY6, Chip8::i_8XY7, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_8XYE,
            ]},

            t_E000: {[Chip8::i_EX9E, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_EXA1]},

            t_F000: {[
                    Chip8::i_FX07, Chip8::i_FX0A, Chip8::i_FX15,
                    Chip8::i_FX18, Chip8::i_FX1E, Chip8::i_FX29,
                    Chip8::i_FX33, Chip8::i_FX55, Chip8::i_FX65,
            ]},

            pc: 0x200,
            opcode: 0,
            i: 0,
            sp: 0,

            gfx: [0; 2048],
            stack: [0; 16],
            v: [0; 16],
            memory: [0; 4096],

            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            draw_flag: false,
            sound_state: true,

            nnn: 0,
            nn:  0,
            n:   0,
            x:   0,
            y:   0,

            fontset: {[
                    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                    0x20, 0x60, 0x20, 0x20, 0x70, // 1
                    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ]},
        };

        for (i, val) in chip.fontset.iter().enumerate() {
            chip.memory[i] = *val;
        }
        chip
    }

    pub fn emulate_cycle(&mut self) {
        let op1 = (self.memory[self.pc as usize]) as u16;
        let op2 = (self.memory[1 + self.pc as usize]) as u16;
        self.opcode = op1 << 8 | op2;
        self.pc += 2;

        self.nnn =  (self.opcode & 0x0FFF)       as u16;   // addr; 12-bit value
        self.nn  =  (self.opcode & 0x00FF)       as u8;    // byte; 8-bit value
        self.n   =  (self.opcode & 0x000F)       as usize; // nibble; 4-bit value
        self.x   = ((self.opcode & 0x0F00) >> 8) as usize; // lower 4 bits of the high byte
        self.y   = ((self.opcode & 0x00F0) >> 4) as usize; // upper 4 bits of the low byte

        self.t_main[((self.opcode & 0xF000) >> 12) as usize](self);

        // Update timers
        if self.sound_state && self.sound_timer > 0 {
            self.sound_timer -= 1;
            println!("PING");
        }

        if self.delay_timer > 0 { self.delay_timer -= 1; }
    }

    // Instructions

    fn i_0000(&mut self) { self.t_0000[self.n](self); }
    fn i_8000(&mut self) { self.t_8000[self.n](self); }
    fn i_E000(&mut self) { self.t_E000[self.nn as usize - 158](self); }

    fn i_F000(&mut self) {
        let x = match self.nn {
            0x007 => 0, 0x00A => 1,
            0x015 => 2, 0x018 => 3,
            0x01E => 4, 0x029 => 5,
            0x033 => 6, 0x055 => 7,
            0x065 => 8,
            _=> {
                self.i_NULL();
                return;
            },
        };

        self.t_F000[x](self);
    }
    
    fn i_NULL(&mut self) {
        eprintln!("Invalid opcode: {} (raw opcode)", self.opcode);
        process::exit(1);
    }
    
    // Clear the screen
    fn i_00E0(&mut self) {
        self.gfx = [0; 2048];
        //self.draw_flag = true;
    }
    
    // Return from a subroutine
    fn i_00EE(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }
    
    // Jump to memory location NNN
    fn i_1NNN(&mut self) { self.pc = self.nnn; }
    
    // Execute subroutine starting at NNN
    fn i_2NNN(&mut self) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = self.nnn;
    }
    
    // Skip next instruction if ...
    fn i_3XNN(&mut self) { if self.v[self.x] == self.nn        { self.pc += 2; }} // VX == NN
    fn i_4XNN(&mut self) { if self.v[self.x] != self.nn        { self.pc += 2; }} // VX != NN
    fn i_5XY0(&mut self) { if self.v[self.x] == self.v[self.y] { self.pc += 2; }} // VX == VY
    
    fn i_6XNN(&mut self) { self.v[self.x]  = self.nn; }        // Set VX == NN
    fn i_7XNN(&mut self) { self.v[self.x] += self.nn; }        // Set VX += NN
    fn i_8XY0(&mut self) { self.v[self.x]  = self.v[self.y]; } // Set VX = VY
    fn i_8XY1(&mut self) { self.v[self.x] |= self.v[self.y]; } // Set VX |= VY
    fn i_8XY2(&mut self) { self.v[self.x] &= self.v[self.y]; } // Set VX &= VY
    fn i_8XY3(&mut self) { self.v[self.x] ^= self.v[self.y]; } // Set VX ^= VY
    
    // Sets VX = VX + VY, set VF = carry
    fn i_8XY4(&mut self) {
        let vxy = self.v[self.x] as u16 + self.v[self.y] as u16;
        self.v[0xF] = (vxy >> 8) as u8; // if VX+VY > 255, then VF = 1, else VF = 0
        self.v[self.x] = vxy as u8;
    }
    
    // Set VX -= VY. set VF = NOT borrow
    fn i_8XY5(&mut self) {
        let vxy = self.v[self.x] as i16 - self.v[self.y] as i16;
        self.v[0xF] = (vxy >= 0) as u8; // If VX < VY, then VF = 0, else VF = 1
        // I don't think this should be here
        self.v[self.x] = vxy as u8; // Should I wrap? IDK
    }
    
    // Set VX = VX SHR 1
    fn i_8XY6(&mut self) {
        self.v[0xF] = self.v[self.x] & 1;
        self.v[self.x] = self.v[self.y] >> 1;
    }
    
    // Set VX = VY - VX. set VF = NOT borrow
    fn i_8XY7(&mut self) {
        let vxy = self.v[self.y] as i16 - self.v[self.x] as i16;
        self.v[0xF] = (vxy >= 0) as u8; // If VY < VX, then VF = 0, else VF = 1
        // I don't think this should be here
        //self.v[self.x] = vxy as u8; // Should I wrap? IDK
    }
    
    // Set VX = VY SHL 1
    fn i_8XYE(&mut self) {
        self.v[0xF] = self.v[self.x] >> 7;
        self.v[self.x] = self.v[self.y] << 1;
    }
    
    // Skip next instruction if VX != VY
    fn i_9XY0(&mut self) { if self.v[self.x] != self.v[self.y] { self.pc += 2; }}

    fn i_ANNN(&mut self) { self.i = self.nnn; } // Store memory address NNN in register I
    fn i_BNNN(&mut self) { self.pc = self.nnn + self.v[0] as u16; } // Jump to location NNN + V0
    fn i_CXNN(&mut self) { self.v[self.x] = rand::thread_rng().gen_range(0, 255) & self.nn; } // Set VX = random byte AND NN
    
    // Display n-byte sprite starting at memory location I at (VX,VY). Set VF = collision
    fn i_DXYN(&mut self) {
        // sprites are always 8 pixels (1 byte) long and between 1 and 15 pixels (up to 2 bytes) high    
        for py in 0..self.n {
            let byte = self.memory[self.i as usize + py];
            for px in 0..8 {
                let pixel = (byte & (0x80 >> px)) >> (7 - px);
                let position = ((self.v[self.x] as usize + px) % 64) + ((self.v[self.y] as usize + py) * 64);

                self.gfx[position] ^= pixel & 1;
                self.v[0xF] = self.gfx[position];
            }
        }
        self.draw_flag = true;
    }

    // Skip next instruction if key with the value of VX is ...
    fn i_EX9E(&mut self) { if self.key[self.v[self.x] as usize] == 1 { self.pc += 2; }} // pressed
    fn i_EXA1(&mut self) { if self.key[self.v[self.x] as usize] == 0 { self.pc += 2; }} // not pressed
    
    fn i_FX07(&mut self) { self.v[self.x] = self.delay_timer; } // Set VX = delay timer value
    
    // Wait for a key press, store the position of the key in VX
    fn i_FX0A(&mut self) {
        loop {
            if let Some(i) = self.key.iter().position(|&val| val == 1) {
                self.v[self.x] = i as u8;
                return;
            }
        }
    }
    
    fn i_FX15(&mut self) { self.delay_timer = self.v[self.x];   } // Set delay timer = VX
    fn i_FX18(&mut self) { self.sound_timer = self.v[self.x];   } // Set sound timer = VX
    fn i_FX1E(&mut self) { self.i += self.v[self.x] as u16;     } // Set I += VX
    fn i_FX29(&mut self) { self.i  = self.v[self.x] as u16 * 5; } // Set I = location of sprite for digit VX
    
    // Store the binary-coded decimal equivalent of the value
    // stored in register VX at address I, I+1, I+2
    fn i_FX33(&mut self) {
        self.memory[self.i as usize]     =  self.v[self.x] / 100;
        self.memory[self.i as usize + 1] = (self.v[self.x] / 10) % 10;
        self.memory[self.i as usize + 2] = (self.v[self.x] % 100) % 10;
    }
    
    // Store registers V0 through VX in memory starting at location I
    fn i_FX55(&mut self) {
        for a in 0..=self.x {
            self.memory[a + self.i as usize] = self.v[a];
        }

        self.i += self.x as u16 + 1;
    }

    // Read registers V0 through VX from memory starting at location I
    fn i_FX65(&mut self) {
        for a in 0..=self.x {
            self.v[a] = self.memory[a + self.i as usize];
        }

        self.i += self.x as u16 + 1;
    }

    pub fn load_game(&mut self, game: &String) {
        let buffer = match fs::read(game) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error: File read error");
                process::exit(1);
            },
        };

        // 512 == 0x200
        if 4096 - 512 > buffer.len() {
            for (i, val) in buffer.iter().enumerate() {
                self.memory[i + 512] = *val;
            }
        } else {
            eprintln!("Error: ROM too big");
            process::exit(1);
        }
    }
}
