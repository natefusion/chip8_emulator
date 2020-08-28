#![allow(non_camel_case_types, non_snake_case)]
use rand::Rng;
use std::fs;

/* Materials:
 * http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
 * https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
 * http://mattmik.com/files/chip8/mastering/chip8.html
 * http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
 *
 * System memory map:
 * 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
 * 0x050-0x0A0 - Used for the builtin 4x5 pixel font set (0-F)
 * 0x200-0xFFF - Program ROM and work RAM
 */

pub struct Chip8 {
    t_main: [fn(&mut Chip8, &Bit); 16],
    t_0000: [fn(&mut Chip8, &Bit); 15],
    t_8000: [fn(&mut Chip8, &Bit); 15],
    t_E000: [fn(&mut Chip8, &Bit); 4],
    t_F000: [fn(&mut Chip8, &Bit); 9],

    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    pc_inc: u16,
    pub gfx: [[u8; 64]; 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    pub key: [u8; 16],
    fontset: [u8; 80],
    pub draw_flag: bool,
    pub sound_state: bool,
}

struct Bit {
    nnn: u16,
    kk: u8,
    n: u8,
    x: usize,
    y: usize,
}

impl Chip8 {
    pub fn initialize() -> Chip8 {
        let mut chip = Chip8 {
            t_main: {
                [
                    Chip8::i_0000, Chip8::i_1NNN, Chip8::i_2NNN, Chip8::i_3XKK,
                    Chip8::i_4XKK, Chip8::i_5XY0, Chip8::i_6XKK, Chip8::i_7XKK,
                    Chip8::i_8000, Chip8::i_9XY0, Chip8::i_ANNN, Chip8::i_BNNN,
                    Chip8::i_CXKK, Chip8::i_DXYN, Chip8::i_E000, Chip8::i_F000,
                ]
            },

            t_0000: {
                [
                    Chip8::i_00E0, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_00EE,
                ]
            },

            t_8000: {
                [
                    Chip8::i_8XY0, Chip8::i_8XY1, Chip8::i_8XY2,
                    Chip8::i_8XY3, Chip8::i_8XY4, Chip8::i_8XY5,
                    Chip8::i_8XY6, Chip8::i_8XY7, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_NULL,
                    Chip8::i_NULL, Chip8::i_NULL, Chip8::i_8XYE,
                ]
            },

            t_E000: { [Chip8::i_EX9E, Chip8::i_NULL, Chip8::i_NULL, Chip8::i_EXA1] },

            t_F000: {
                [
                    Chip8::i_FX07, Chip8::i_FX0A, Chip8::i_FX15,
                    Chip8::i_FX18, Chip8::i_FX1E, Chip8::i_FX29,
                    Chip8::i_FX33, Chip8::i_FX55, Chip8::i_FX65,
                ]
            },

            pc: 0x200,
            pc_inc: 2,
            opcode: 0,
            i: 0,
            sp: 0,

            gfx: [[0; 64]; 32],
            stack: [0; 16],
            v: [0; 16],
            memory: [0; 4096],

            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
            draw_flag: false,
            sound_state: true,

            fontset: {
                [
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
                ]
            },
        };

        for (i, val) in chip.fontset.iter().enumerate() {
            chip.memory[i] = *val;
        }
        chip
    }

    fn fetch(&mut self) {
        let op1 = (self.memory[self.pc as usize]) as u16;
        let op2 = (self.memory[1 + self.pc as usize]) as u16;
        self.opcode = op1 << 8 | op2;
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch();

        let bit = Bit {
            nnn: (self.opcode & 0x0FFF) as u16,        // addr; 12-bit value
            kk: (self.opcode & 0x00FF) as u8,          // byte; 8-bit value
            n: (self.opcode & 0x000F) as u8,           // nibble; 4-bit value
            x: ((self.opcode & 0x0F00) >> 8) as usize, // lower 4 bits of the high byte
            y: ((self.opcode & 0x00F0) >> 4) as usize, // upper 4 bits of the low byte
        };

        self.t_main[((self.opcode & 0xF000) >> 12) as usize](self, &bit);

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 && self.sound_state {
                println!("PRETEND THIS IS A SOUND");
            }
            self.sound_timer -= 1;
        }

        self.pc += self.pc_inc;
        self.pc_inc = 2;
    }

    // Instructions

    fn i_0000(&mut self, bit: &Bit) {
        self.t_0000[(self.opcode & 0x000F) as usize](self, bit);
    }
    fn i_8000(&mut self, bit: &Bit) {
        self.t_8000[(self.opcode & 0x000F) as usize](self, bit);
    }
    fn i_E000(&mut self, bit: &Bit) {
        self.t_E000[(self.opcode & 0x00FF) as usize - 158](self, bit);
    }

    fn i_F000(&mut self, bit: &Bit) {
        match self.opcode & 0x00FF {
            0x007 => self.t_F000[0](self, bit),
            0x00A => self.t_F000[1](self, bit),
            0x015 => self.t_F000[2](self, bit),
            0x018 => self.t_F000[3](self, bit),
            0x01E => self.t_F000[4](self, bit),
            0x029 => self.t_F000[5](self, bit),
            0x033 => self.t_F000[6](self, bit),
            0x055 => self.t_F000[7](self, bit),
            0x065 => self.t_F000[8](self, bit),
            _ => self.i_NULL(bit),
        }
    }

    fn i_NULL(&mut self, _bit: &Bit) {
        println!("Invalid opcode: {} (raw opcode)", self.opcode);
        std::process::exit(1);
    }

    // Clears the screen
    fn i_00E0(&mut self, _bit: &Bit) {
        for i in self.gfx.iter_mut() {
            for val in i.iter_mut() {
                *val = 0;
            }
        }
        self.draw_flag = true;
    }
    // Returns from the subroutine
    fn i_00EE(&mut self, _bit: &Bit) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }
    // Jump to location NNN
    fn i_1NNN(&mut self, bit: &Bit) {
        self.pc = bit.nnn;
        self.pc_inc = 0;
    }
    // Execute subroutine starting at NNN
    fn i_2NNN(&mut self, bit: &Bit) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = bit.nnn;
        self.pc_inc = 0;
    }
    // Skip next instruction if VX == KK
    fn i_3XKK(&mut self, bit: &Bit) {
        if self.v[bit.x] == bit.kk {
            self.pc_inc = 4;
        }
    }
    // Skip next instruction if VX != KK
    fn i_4XKK(&mut self, bit: &Bit) {
        if self.v[bit.x] != bit.kk {
            self.pc_inc = 4;
        }
    }
    // Skip next instruction if VX == VY
    fn i_5XY0(&mut self, bit: &Bit) {
        if self.v[bit.x] == self.v[bit.y] {
            self.pc_inc = 4;
        }
    }
    // Set VX == KK
    fn i_6XKK(&mut self, bit: &Bit) {
        self.v[bit.x] = bit.kk;
    }
    // Set VX += KK
    fn i_7XKK(&mut self, bit: &Bit) {
        self.v[bit.x] = self.v[bit.x].wrapping_add(bit.kk);
    }
    // Set VX = VY
    fn i_8XY0(&mut self, bit: &Bit) {
        self.v[bit.x] = self.v[bit.y];
    }
    // Set VX |= VY
    fn i_8XY1(&mut self, bit: &Bit) {
        self.v[bit.x] |= self.v[bit.y];
    }
    // Set VX &= VY
    fn i_8XY2(&mut self, bit: &Bit) {
        self.v[bit.x] &= self.v[bit.y];
    }
    // Set VX ^= VY
    fn i_8XY3(&mut self, bit: &Bit) {
        self.v[bit.x] ^= self.v[bit.y];
    }
    // Sets VX = VX + VY, set VF = carry
    fn i_8XY4(&mut self, bit: &Bit) {
        let vxy = (self.v[bit.x].wrapping_add(self.v[bit.y])) as u16;
        if vxy > 255 {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[bit.x] = (vxy & 0x00FF) as u8;
    }
    // Set VX -= VY. set VF = NOT borrow
    fn i_8XY5(&mut self, bit: &Bit) {
        if self.v[bit.x] > self.v[bit.y] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[bit.x] = self.v[bit.x].wrapping_sub(self.v[bit.y]);
    }
    // Set VX = VX SHR 1
    fn i_8XY6(&mut self, bit: &Bit) {
        self.v[0xF] = self.v[bit.x] & 1;
        self.v[bit.x] /= 2;
    }
    // Set VX = VY - VX. set VF = NOT borrow
    fn i_8XY7(&mut self, bit: &Bit) {
        if self.v[bit.y] > self.v[bit.x] {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[bit.x] = self.v[bit.y] - self.v[bit.x];
    }
    // Set VX = VX SHL 1
    fn i_8XYE(&mut self, bit: &Bit) {
        self.v[0xF] = self.v[bit.x] >> 7;
        self.v[bit.x] *= 2;
    }
    // Skip next instruction if VX != VY
    fn i_9XY0(&mut self, bit: &Bit) {
        if self.v[bit.x] != self.v[bit.y] {
            self.pc_inc = 4;
        }
    }
    // Store memory address NNN in register I
    fn i_ANNN(&mut self, bit: &Bit) {
        self.i = bit.nnn;
    }
    // Jump to location NNN + V0
    fn i_BNNN(&mut self, bit: &Bit) {
        self.pc = bit.nnn + self.v[0] as u16;
        self.pc_inc = 0;
    }
    // Set VX = random byte AND KK
    fn i_CXKK(&mut self, bit: &Bit) {
        self.v[bit.x] = rand::thread_rng().gen_range(0, 255) & bit.kk;
    }
    // Display n-byte sprite starting at memory location I at (VX,VY). Set VF = collision
    fn i_DXYN(&mut self, bit: &Bit) {
        let x = self.v[bit.x] as u16;
        let y = self.v[bit.y] as u16;
        let height = bit.n as u16;
        let mut pixel: u16;

        let gfx_c = self.gfx[0].len() - 1;
        let gfx_r = self.gfx.len() - 1;

        self.v[0xF] = 0;
        for yline in 0..height {
            pixel = self.memory[(self.i + yline) as usize] as u16;
            for xline in 0..8 {
                if pixel & (0x80 >> xline) != 0 {
                    let mut row = (y + yline) as usize;
                    let mut col = (x + xline) as usize;

                    // wraps values for user-movable items that try to move out-of-bounds
                    if row > gfx_r { row = gfx_r; }
                    if col > gfx_c { col = gfx_c; }

                    if self.gfx[row][col] == 1 { 
                        self.v[0xF] = 1;
                    }
                    self.gfx[row][col] ^= 1;
                }
            }
        }
        self.draw_flag = true;
    }
    // Skip next instruction if key with the value of VX is pressed
    fn i_EX9E(&mut self, bit: &Bit) {
        if self.key[self.v[bit.x] as usize] != 0 {
            self.pc_inc = 4;
        }
    }
    // Skip next instruction if key with the value of VX is not pressed
    fn i_EXA1(&mut self, bit: &Bit) {
        if self.key[self.v[bit.x] as usize] == 0 {
            self.pc_inc = 4;
        }
    }
    // Set VX = delay timer value
    fn i_FX07(&mut self, bit: &Bit) {
        self.v[bit.x] = self.delay_timer;
    }
    // Wait for a key press, store the avlue of the key in VX
    fn i_FX0A(&mut self, bit: &Bit) {
        'key: loop {
            for val in self.key.iter() {
                if *val == 1 {
                    self.v[bit.x] = *val;
                    break 'key;
                }
            }
        }
    }
    // Set delay timer = VX
    fn i_FX15(&mut self, bit: &Bit) {
        self.delay_timer = self.v[bit.x];
    }
    // Set sound timer = VX
    fn i_FX18(&mut self, bit: &Bit) {
        self.sound_timer = self.v[bit.x];
    }
    // Set I += VX
    fn i_FX1E(&mut self, bit: &Bit) {
        self.i += self.v[bit.x] as u16;
    }
    // Set I = location of sprite for digit VX
    fn i_FX29(&mut self, bit: &Bit) {
        self.i = 5 * self.v[bit.x] as u16; // Sprites are 5 bytes in height
    }
    // Store the binary-coded decimal equivalent of the value
    // stored in register VX at address I, I+1, I+2
    fn i_FX33(&mut self, bit: &Bit) {
        self.memory[self.i as usize] = self.v[bit.x] / 100;
        self.memory[1 + self.i as usize] = (self.v[bit.x] / 10) % 10;
        self.memory[2 + self.i as usize] = (self.v[bit.x] % 100) % 10;
    }
    // Store registers V0 through VX in memory starting at location I
    fn i_FX55(&mut self, bit: &Bit) {
        for a in 0..=bit.x {
            self.memory[a + self.i as usize] = self.v[a];
        }
    }
    // Read registers V0 through VX from memory starting at location I
    fn i_FX65(&mut self, bit: &Bit) {
        for a in 0..=bit.x {
            self.v[a] = self.memory[a + self.i as usize];
        }
    }

    pub fn load_game(&mut self, game: &String) {
        let buffer = fs::read(game).expect("File read error");

        // 512 == 0x200
        if 4096 - 512 > buffer.len() {
            for (i, val) in buffer.iter().enumerate() {
                self.memory[i + 512] = *val;
            }
        } else {
            println!("Error: ROM too big");
            std::process::exit(1);
        }
    }
}
