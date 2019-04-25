extern crate rand;
use rand::prelude::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use super::mem::*;
use super::gfx::*;

type Instruction = fn(&mut CPU, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8);

pub struct CPU {
    pub screen: Screen,
    pub input: [bool; 16],
    mem: Memory,
    regs: Registers,
    stack: Stack,
    pc: u16,
    rng: ThreadRng
}

impl CPU {
    pub fn new() -> CPU {
        let mut result = CPU {
            screen: Screen::new(),
            input: [false; 16],
            mem: Memory::new(),
            regs: Registers::new(),
            stack: Stack::new(),
            pc: 512,
            rng: rand::thread_rng()
        };

        //Load standard hex char sprites
        let hex_chars : [u8; 16*5] = [
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
			0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for hex_char in hex_chars.iter().enumerate() {
            result.mem.write(hex_char.0 as u16, *hex_char.1);
        }

        result
    }

    pub fn load_rom(&mut self, path : &str) {
        let mut f = File::open(path);
        match f {
            Ok(mut f) => {
                let mut buffer = Vec::new();

                let success = f.read_to_end(&mut buffer);
                match success {
                    Ok(e) => {
                        for i in 0..e {
                            self.mem.write(512 + i as u16, buffer[i]);
                        }
                    }
                    Err(e) => println!("Could not load rom to memory.")
                }
            },
            Err(f) => println!("Could not open file.")
        }
    }

    pub fn step(&mut self) {
        //Fetch opcode
        let opcode = (self.mem.read(self.pc) as u16) << 8 | self.mem.read(self.pc + 1) as u16;

        //Decode opcode
        let op  = ((opcode & 0xF000) >> 12) as u8;
        let nnn =  opcode & 0x0FFF;
        let n   = (opcode & 0x000F) as u8;
        let x   = ((opcode & 0x0F00) >> 8) as u8;
        let y   = ((opcode & 0x00F0) >> 4) as u8;
        let kk  = (opcode & 0x00FF) as u8;

        //Fetch instruction
        let instr : Instruction = match op {
            0x0 => match nnn {
                0x0E0 => Self::op_cls,
                0x0EE => Self::op_ret,
                _ => Self::op_sys
            },
            0x1 => Self::op_jp,
            0x2 => Self::op_call,
            0x3 => Self::op_se,
            0x4 => Self::op_sne,
            0x5 => match n {
                0x0 => Self::op_se,
                _ => Self::op_err
            },
            0x6 => Self::op_ld,
            0x7 => Self::op_add,
            0x8 => match n {
                0x0 => Self::op_ld,
                0x1 => Self::op_or,
                0x2 => Self::op_and,
                0x3 => Self::op_xor,
                0x4 => Self::op_add,
                0x5 => Self::op_sub,
                0x6 => Self::op_shr,
                0x7 => Self::op_subn,
                0xE => Self::op_shl,
                _=> Self::op_err
            },
            0x9 => Self::op_sne,
            0xA => Self::op_ld,
            0xB => Self::op_jp,
            0xC => Self::op_rnd,
            0xD => Self::op_drw,
            0xE => match kk {
                0x9E => Self::op_skp,
                0xA1 => Self::op_sknp,
                _ => Self::op_err
            },
            0xF => match kk {
                0x07 | 0x0A | 0x15 | 0x18 | 0x29 | 0x33 | 0x55 | 0x65 => Self::op_ld,
                0x1E => Self::op_add,
                _ => Self::op_err
            },
            _ => Self::op_err
        };

        //Execute instruction
        instr(self, op, nnn, n, x, y, kk);

        //FIXME
        /*println!("Opcode: {:x}", opcode);
        println!("op={:x}, nnn={:x}, n={:x}, x={:x}, y={:x}", op, nnn, n, x, y);
        print!("v=[");
        print!("{:x}", self.regs.getv(0));
        for i in 1..16 {
            print!(", {:x}", self.regs.getv(i));
        }
        println!("]");
        println!("i={:x}", self.regs.i);
        println!("");*/

        //Increase program counter
        self.pc += 2;
    }

    pub fn update_timers(&mut self) {
        //Update timers
        if self.regs.dt > 0 {
            self.regs.dt -= 1;
        }
        if self.regs.st > 0 {
            println!("Beep");
            self.regs.st -= 1;
        }
    }

    pub fn op_err(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        panic!("Invalid opcode");
    }

    pub fn op_cls(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.screen.clear_screen();
    }

    pub fn op_ret(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.pc = self.stack.pop();
    }

    pub fn op_sys(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        //Nothing
    }

    pub fn op_jp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (op == 1) {
            self.pc = nnn - 2;
        }
        else if (op == 0xB) {
            self.pc = nnn + self.regs.getv(0) as u16 - 2;
        }
    }

    pub fn op_call(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.stack.push(self.pc);
        self.pc = nnn - 2;
    }

    pub fn op_se(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 3 {
            if self.regs.getv(x) == kk {
                self.pc += 2;
            }
        }
        else if op == 5 {
            if self.regs.getv(x) == self.regs.getv(y) {
                self.pc += 2;
            }
        }
    }

    pub fn op_sne(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 4 {
            if self.regs.getv(x) != kk {
                self.pc += 2;
            }
        }
        else if op == 9 {
            if self.regs.getv(x) != self.regs.getv(y) {
                self.pc += 2;
            }
        }
    }

    pub fn op_ld(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 6 {
            self.regs.setv(x, kk);
        }
        else if op == 8 {
            self.regs.setv(x, self.regs.getv(y));
        }
        else if op == 0xA {
            self.regs.i = nnn;
        }
        else if op == 0xF {
            match kk {
                0x07 => self.regs.setv(x, self.regs.dt),
                0x0A => {}, //FIXME
                0x15 => self.regs.dt = self.regs.getv(x),
                0x18 => self.regs.st = self.regs.getv(x),
                0x29 => self.regs.i = self.regs.getv(x) as u16 * 5,
                0x33 => {
                    let vx = self.regs.getv(x) as i32;
                    self.mem.write(self.regs.i, ((vx % 10_i32.pow(3)) / 10_i32.pow(3-1)) as u8);
                    self.mem.write(self.regs.i+1, ((vx % 10_i32.pow(2)) / 10_i32.pow(2-1)) as u8);
                    self.mem.write(self.regs.i+2, ((vx % 10_i32.pow(1)) / 10_i32.pow(1-1)) as u8);
                },
                0x55 => for i in 0..x as u16 {
                    self.mem.write(self.regs.i + i, self.regs.getv(i as u8));
                },
                0x65 => for i in 0..x as u16 {
                    self.regs.setv(i as u8, self.mem.read(self.regs.i + i));
                },
                _ => {}
            }
        }
    }

    pub fn op_add(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 7 {
            self.regs.setv(x, self.regs.getv(x).wrapping_add(kk));
        }
        else if op == 8 {
            let result : u16 = self.regs.getv(x) as u16 + self.regs.getv(y) as u16;
            self.regs.setv(0xF, if result > 255 { 1 } else { 0 });
            self.regs.setv(x, (result & 0xFF) as u8);
        }
        else if op == 0xF && kk == 0x1E {
            self.regs.i += self.regs.getv(x) as u16;
        }
    }

    pub fn op_or(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) | self.regs.getv(y));
        }
    }

    pub fn op_xor(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) ^ self.regs.getv(y));
        }
    }

    pub fn op_and(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) & self.regs.getv(y));
        }
    }

    pub fn op_sub(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(x) > self.regs.getv(y) { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(x).wrapping_sub(self.regs.getv(y)));
    }

    pub fn op_shr(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(x) & 1 == 1 { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(x).wrapping_div(2));
    }

    pub fn op_subn(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(y) > self.regs.getv(x) { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(y).wrapping_sub(self.regs.getv(x)));
    }

    pub fn op_shl(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(x) & 0b10000000 != 0 { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(x).wrapping_mul(2));
    }

    pub fn op_rnd(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        let rnd: u8 = self.rng.gen();
        self.regs.setv(x, rnd & kk);
    }

    pub fn op_drw(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        let sprite_height = n as usize;
        let mut sprite = vec![0; sprite_height];
        for i in 0..sprite_height {
            sprite[i] = self.mem.read(self.regs.i + i as u16);
        }
        self.screen.draw_sprite(self.regs.getv(x), self.regs.getv(y), &sprite);
    }

    pub fn op_skp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (self.input[self.regs.getv(x) as usize]) {
            self.pc += 2;
        }
    }

    pub fn op_sknp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (!self.input[self.regs.getv(x) as usize]) {
            self.pc += 2;
        }
    }
}