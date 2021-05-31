#![allow(non_camel_case_types, non_snake_case)]
use rand::Rng;
use std::{fs::File, io::{Seek, SeekFrom,Read},process,time::Duration,thread::sleep};

/* Materials:
 * http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
 * http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
 *
 * System memory map:
 * 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
 * 0x050-0x0A0 - Used for the builtin 4x5 pixel font set (0-F)
 * 0x200-0xFFF - Program ROM and work RAM
 */

const W:     usize = 64;
const H:     usize = 32;
const DELAY: u64   = 16_666;
const MEM:   usize = 4096;

pub struct Chip8 {
    // Function pointer tables that hold a reference to all instructions    
    t_main: [fn(&mut Self); 16],
    t_0000: [fn(&mut Self); 2],
    t_8000: [fn(&mut Self); 15],
    t_E000: [fn(&mut Self); 2],
    t_F000: [fn(&mut Self); 9],

    // Caries the instruction to be executed; Each instruction is two bytes long
    opcode: u16,

    /* System memory; 4096 bytes
     * Memory map:
     * 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
     * 0x050-0x0A0 - Used for the builtin 4x5 pixel font set (0-F)
     * 0x200-0xFFF - Program ROM and work RAM
     */
    memory: [u8; MEM],

    // Cpu registers; Used for storing up to 15 different values; V[0-F] (VF for 'carry flag')
    v: [u8; 16],

    // Index register; Holds a specified memory index for later use 
    i: u16,
    
    // Program counter; for incrementing through instructions
    pc: u16,

    // Delay timer; Will delay the next instruction from being executed for multiples of 1/60s
    dt: u8,

    // Sound timer; Will play a sound for multiples of 1/60s
    st: u8,

    // Turns sound on/off
    pub sound_state: bool,

    // Stack; Allows for 16 different subroutines at any one time
    stack: [u16; 16],

    // Stack pointer; for navigation through the stack
    sp: u16,

    // Screen is 64px by 32px
    pub gfx: [u8; W*H],

    // The display will only be updated when this is true
    pub draw_flag: bool,
    
    // Holds keyboard state; 16 keys available
    pub keys: [u8; 16],

    // Aliases for currently used values
    nnn: u16,   // address; 12-bit value              
    nn:  u8,    // byte;    8-bit value               
    n:   usize, // nibble;  4-bit value             
    x:   usize, // nibble: lower 4 bits of the high byte
    y:   usize, // nibble: upper 4 bits of the low byte 
}

impl Chip8 {
    pub fn initialize() -> Self {
        let mut chip = Self {
            t_main: [Self::i_0000, Self::i_1NNN, Self::i_2NNN, Self::i_3XNN, Self::i_4XNN,
                     Self::i_5XY0, Self::i_6XNN, Self::i_7XNN, Self::i_8000, Self::i_9XY0,
                     Self::i_ANNN, Self::i_BNNN, Self::i_CXNN, Self::i_DXYN, Self::i_E000, Self::i_F000],
            
            t_0000: [Self::i_00E0, Self::i_00EE],
            
            t_8000: [Self::i_8XY0, Self::i_8XY1, Self::i_8XY2, Self::i_8XY3, Self::i_8XY4,
                     Self::i_8XY5, Self::i_8XY6, Self::i_8XY7, Self::i_NULL, Self::i_NULL,
                     Self::i_NULL, Self::i_NULL, Self::i_NULL, Self::i_NULL, Self::i_8XYE],
            
            t_E000: [Self::i_EX9E, Self::i_EXA1],
            
            t_F000: [Self::i_FX07, Self::i_FX0A, Self::i_FX15, Self::i_FX18, Self::i_FX1E,
                     Self::i_FX29, Self::i_FX33, Self::i_FX55, Self::i_FX65],

            opcode: 0,
            memory: [0; MEM],
            v: [0; 16],
            i: 0,
            pc: 0x200, // programs are loaded at memory[0x200]
            dt: 0,
            st: 0,
            sound_state: true,
            stack: [0; 16],
            sp: 0,
            gfx: [0; W*H],
            draw_flag: false,            
            keys: [0; 16],
            nnn: 0,
            nn:  0,
            n:   0,
            x:   0,
            y:   0,
        };

        [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
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
        ].iter().enumerate().for_each(|(i, val)| chip.memory[i] = *val);

        chip
    }

    pub fn emulate_cycle(&mut self) {
        self.draw_flag = false;
        let op1 = self.memory[self.pc as usize] as u16;
        let op2 = self.memory[self.pc as usize + 1] as u16;
        self.opcode = op1 << 8 | op2;
        self.pc += 2;

        self.nnn =  (self.opcode & 0xFFF)       as u16;   // addr; 12-bit value
        self.nn  =  (self.opcode & 0xFF)        as u8;    // byte; 8-bit value
        self.n   =  (self.opcode & 0xF)         as usize; // nibble; 4-bit value
        self.x   = ((self.opcode & 0xF00) >> 8) as usize; // lower 4 bits of the high byte
        self.y   = ((self.opcode & 0xF0) >> 4)  as usize; // upper 4 bits of the low byte

        self.t_main[((self.opcode & 0xF000) >> 12) as usize](self);

        // Update timers
        if self.sound_state && self.st > 0 {
            self.st -= 1;
            println!("PING");
            sleep(Duration::from_micros(DELAY));
        }

        if self.dt > 0 {
            self.dt -= 1;
            sleep(Duration::from_micros(DELAY));
        }
    }

    // Instructions

    fn i_0000(&mut self) { self.t_0000[self.n >> 3](self); }
    fn i_8000(&mut self) { self.t_8000[self.n     ](self); }
    fn i_E000(&mut self) { self.t_E000[self.y  - 9](self); }

    fn i_F000(&mut self) {
        let x = match self.nn {
            //7 + 3 + 11 + 3 + 6 + 11 + 10 + 22 + 28 = 0x65
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
        eprintln!("Unrecognized opcode: {:X}", self.opcode);
        process::exit(1);
    }
    
    // Clear the screen
    fn i_00E0(&mut self) {
        self.gfx = [0; W*H];
        self.draw_flag = true;
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
        let sum = self.v[self.x] as u16 + self.v[self.y] as u16;
        self.v[0xF] = (sum > 0xFF) as u8; // VF = 1 if carry occurs, 0 if not
        self.v[self.x] = sum as u8;
    }
    
    // Set VX -= VY. set VF = NOT borrow
    fn i_8XY5(&mut self) {
        let diff = self.v[self.x] as i8 - self.v[self.y] as i8;
        self.v[0xF] = (diff > 0) as u8; // If VX < VY, then VF = 0, else VF = 1
        self.v[self.x] = diff as u8;
    }
    
    // Set VX = VX SHR 1
    fn i_8XY6(&mut self) {
        self.v[0xF] = self.v[self.x] & 1;
        self.v[self.x] = self.v[self.y] >> 1;
    }
    
    // Set VX = VY - VX. set VF = NOT borrow
    fn i_8XY7(&mut self) {
        let diff = self.v[self.y] as i8 - self.v[self.x] as i8;
        self.v[0xF] = (diff > 0) as u8; // If VY < VX, then VF = 0, else VF = 1
        self.v[self.x] = diff as u8;
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
                let position = ((self.v[self.x] as usize + px) % W) + (((self.v[self.y] as usize + py) % H) * W);
                
                let oldpixel = self.gfx[position];
                self.gfx[position] ^= pixel & 1;
                self.v[0xF] = oldpixel ^ self.gfx[position];

            }
        }
        self.draw_flag = true;
    }

    // Skip next instruction if key with the value of VX is ...
    fn i_EX9E(&mut self) { if self.keys[self.v[self.x] as usize] == 1 { self.pc += 2; }} // pressed
    fn i_EXA1(&mut self) { if self.keys[self.v[self.x] as usize] == 0 { self.pc += 2; }} // not pressed
    
    fn i_FX07(&mut self) { self.v[self.x] = self.dt; } // Set VX = delay timer value
    
    // Wait for a key press, store the position of the key in VX
    fn i_FX0A(&mut self) {
        match self.keys.iter().position(|&val| val == 1) {
            Some(i) => self.v[self.x] = i as u8,
            None => self.pc -= 2,
        }
    }
    
    fn i_FX15(&mut self) { self.dt = self.v[self.x];   } // Set delay timer = VX
    fn i_FX18(&mut self) { self.st = self.v[self.x];   } // Set sound timer = VX
    fn i_FX1E(&mut self) { self.i += self.v[self.x] as u16;     } // Set I += VX
    fn i_FX29(&mut self) { self.i  = self.v[self.x] as u16 * 5; } // Set I = location of sprite for digit VX
    
    // Store the binary-coded decimal equivalent of the value
    // stored in register VX at address I, I+1, I+2
    fn i_FX33(&mut self) {
        self.memory[self.i as usize]     =  self.v[self.x] / 100;
        self.memory[self.i as usize + 1] = (self.v[self.x] / 10) % 10;
        self.memory[self.i as usize + 2] = (self.v[self.x] % 100) % 10;
    }
    
    // Store/read registers V0 through VX in/from memory starting at location I respectively
    fn i_FX55(&mut self) { for a in 0..=self.x { self.memory[a + self.i as usize] = self.v[a]; }}
    fn i_FX65(&mut self) { for a in 0..=self.x { self.v[a] = self.memory[a + self.i as usize]; }}

    pub fn load_game(&mut self, game: &mut File) {
        let mut buffer = vec![];
        game.seek(SeekFrom::Start(0)).unwrap();
        game.read_to_end(&mut buffer).unwrap();

        if buffer.len()-1 <= MEM - 512 {
            let mut byte = buffer.iter();

            for val in self.memory.iter_mut().skip(512) {
                *val = match byte.next() {
                    Some(b) => *b,
                    None => 0,
                }
            }
        } else {
            eprintln!("Error: ROM too big.\nYour ROM size: {} B\nMax size: 3584 B", buffer.len()-1);
            process::exit(1);
        }
    }
}
