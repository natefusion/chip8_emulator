#![allow(non_camel_case_types, non_snake_case)]
use rand::Rng;
use std::{fs::File, io::{Seek, SeekFrom,Read}, process};

/* Materials:
 * http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
 * http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
 */

const W:     usize = 64;
const H:     usize = 32;
const MEM:   usize = 4096;
const START: usize = 0x200;

pub struct Chip8 {
    // Carries the instruction to be executed; Each instruction is two bytes long
    opcode: u16,

    /* Memory map:
     * 0x000-0x1FF - Reserved for interpreter. Usually used for builtin sprites
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
    pub dt: u8,

    // Sound timer; Will play a sound for multiples of 1/60s
    pub st: u8,

    // Stack; Allows for 16 different subroutines at any one time
    stack: [u16; 16],

    // Stack pointer; for navigation through the stack
    sp: u16,

    // Screen is 64px by 32px
    pub gfx: [[u8; H]; W],

    // The display will only be updated when this is true
    pub draw_flag: bool,

    // True when waiting for a key press
    pub waiting: bool,

    // Holds keyboard state; 16 keys available
    pub keys: [u8; 16],
}

impl Chip8 {
    pub fn initialize() -> Self {
        let mut chip = Self {
            opcode: 0,
            memory: [0; MEM],
            v: [0; 16],
            i: 0,
            pc: START as u16, // programs are loaded at memory[0x200]
            dt: 0,
            st: 0,
            stack: [0; 16],
            sp: 0,
            gfx: [[0; H]; W],
            draw_flag: false,
            waiting: false,
            keys: [0; 16],
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
        let op1 = self.memory[self.pc as usize] as u16;
        let op2 = self.memory[self.pc as usize + 1] as u16;
        self.opcode = op1 << 8 | op2;
        self.pc += 2;

        let nnn =  (self.opcode & 0xFFF)        as u16;   // addr; 12-bit value
        let nn  =  (self.opcode & 0xFF)         as u8;    // byte; 8-bit value
        let n   =  (self.opcode & 0xF)          as usize; // nibble; 4-bit value
        let x   = ((self.opcode & 0xF00) >> 8)  as usize; // lower 4 bits of the high byte
        let y   = ((self.opcode & 0xF0)  >> 4)  as usize; // upper 4 bits of the low byte
        let w   = (self.opcode & 0xF000) >> 12  as usize; // Highest byte

        match (w, x, y, n) {
            (0,0,0xE,0x0) => self.gfx = [[0; H]; W],
            (0,0,0xE,0xE) => { self.pc = self.stack[self.sp as usize];
                               self.sp -= 1; },
            
            (1,_,_,_) => self.pc = nnn,
            
            (2,_,_,_) => { self.sp += 1;
                              self.stack[self.sp as usize] = self.pc;
                              self.pc = nnn; },
            
            (3,_,_,_) => if self.v[x] == nn        { self.pc += 2; },
            (4,_,_,_) => if self.v[x] != nn        { self.pc += 2; },
            (5,_,_,_) => if self.v[x] == self.v[y] { self.pc += 2; },
            
            (6,_,_,_) => self.v[x]  = nn,
            (7,_,_,_) => self.v[x] += nn,

            (8,_,_,0) => self.v[x]  = self.v[y],
            (8,_,_,1) => self.v[x] |= self.v[y],
            (8,_,_,2) => self.v[x] &= self.v[y],
            (8,_,_,3) => self.v[x] ^= self.v[y],
           
            (8,_,_,4) => { let sum = self.v[x] as u16 + self.v[y] as u16;
                           self.v[0xF] = (sum > 0xFF) as u8;
                           self.v[x] = sum as u8; }
            
            (8,_,_,5) => { let diff = self.v[x] as i8 - self.v[y] as i8;
                           self.v[0xF] = (diff > 0) as u8;
                           self.v[x] = diff as u8; },
            
            (8,_,_,6) => { self.v[0xF] = self.v[x] & 1;
                           self.v[x] = self.v[y] >> 1; },
            
            (8,_,_,7) => { let diff = self.v[y] as i8 - self.v[x] as i8;
                           self.v[0xF] = (diff > 0) as u8;
                           self.v[x] = diff as u8; },
            
            (8,_,_,0xE) => { self.v[0xF] = self.v[x] >> 7;
                             self.v[x] = self.v[y] << 1; },
            
            (9,_,_,0) => if self.v[x] != self.v[y] { self.pc += 2; },
            
            (0xA,_,_,_) => self.i = nnn,
            (0xB,_,_,_) => self.pc = nnn + self.v[0] as u16,
            (0xC,_,_,_) => self.v[x] = rand::thread_rng().gen_range(0, 255) & nn,
            
            (0xD,_,_,_) => { self.draw_flag = true;
                             for py in 0..n {
                                 let byte = self.memory[self.i as usize + py];
                                 for px in 0..8 {
                                     // zeros should not be drawn
                                     if (byte >> (7 - px)) & 1 == 0 { continue; };
                                     
                                     let x_coor = (px + self.v[x] as usize) % W;
                                     let y_coor = (py + self.v[y] as usize) % H;
                                     let pixel = self.gfx[x_coor][y_coor];

                                     self.gfx[x_coor][y_coor] = if pixel == 0 { 1 } else { 0 };
                                     self.v[0xF] = pixel;
                                 }} },
            
            (0xE,_,0x9,0xE) => if self.keys[self.v[x] as usize] == 1 { self.pc += 2; },
            (0xE,_,0xA,0x1) => if self.keys[self.v[x] as usize] == 0 { self.pc += 2; },
            
            (0xF,_,0,7) => self.v[x] = self.dt,
            
            (0xF,_,0,0xA) => match self.keys.iter().position(|&v| v == 1) {
                Some(i) => { self.waiting = false; self.v[x] = i as u8; },
                None    => self.waiting = true },
            
            (0xF,_,1,0x5) => self.dt = self.v[x],
            (0xF,_,1,0x8) => self.st = self.v[x],
            (0xF,_,1,0xE) => self.i += self.v[x] as u16,
            (0xF,_,2,0x9) => self.i  = self.v[x] as u16 * 5, // Sprites are 5 bytes in length
            
            (0xF,_,3,3) => { self.memory[self.i as usize]     = (self.v[x] / 100) % 10;
                             self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                             self.memory[self.i as usize + 2] = self.v[x] % 10; },
            
            (0xF,_,5,5) => for a in 0..=x { self.memory[a + self.i as usize] = self.v[a]; },
            (0xF,_,6,5) => for a in 0..=x { self.v[a] = self.memory[a + self.i as usize]; },

            _ => println!("Unknown opcode: 0x{:x}", self.opcode),
        }       
    }

    pub fn load_game(&mut self, game: &mut File) {
        let mut buffer = vec![];
        game.seek(SeekFrom::Start(0)).unwrap();
        game.read_to_end(&mut buffer).unwrap();

        if buffer.len()-1 > MEM - START {
            eprintln!("Error: ROM too big.\nYour ROM size: {} B\nMax size: {} B", buffer.len()-1, MEM - START);
            process::exit(1);
        }
        
        let mut byte = buffer.iter();
        
        for val in self.memory.iter_mut().skip(START) {
            *val = match byte.next() {
                Some(b) => *b,
                None => 0,
            }
        }
        
        // reset game state
        self.opcode = 0;
        self.v = [0; 16];
        self.i = 0;
        self.pc = START as u16;
        self.dt = 0;
        self.st = 0;
        self.stack = [0; 16];
        self.sp = 0;
        self.gfx = [[0; H]; W];
        self.draw_flag = false;
        self.keys = [0;16];
    }
}
