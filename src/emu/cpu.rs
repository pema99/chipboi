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
    rng: ThreadRng,

    pub legacy_ld_sta: bool,
    pub legacy_shl_shr: bool
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            screen: Screen::new(),
            input: [false; 16],
            mem: Memory::new(),
            regs: Registers::new(),
            stack: Stack::new(),
            pc: 512,
            rng: rand::thread_rng(),

            legacy_ld_sta: false,
            legacy_shl_shr: false
        }
    }

    pub fn load_rom(&mut self, path : &str) -> Result<(), &'static str> {
        let mut f = File::open(path).map_err(|_| "Could not open file.")?;

        let mut buffer = Vec::new();
        let e = f.read_to_end(&mut buffer).map_err(|_| "Could not load rom to memory.")?;
        
        for i in 0..e {
            self.mem.write(512 + i as u16, buffer[i]);
        }

        Ok(())
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

        //println!("{} {:04X?} {} {:?} {:?}", self.pc, opcode, self.regs.i, self.stack.data, self.regs.v);

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

    fn op_err(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        panic!("Invalid opcode");
    }

    fn op_cls(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.screen.clear_screen();
    }

    fn op_ret(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.pc = self.stack.pop();
    }

    fn op_sys(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        //Nothing
    }

    fn op_jp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (op == 1) {
            self.pc = nnn - 2;
        }
        else if (op == 0xB) {
            self.pc = nnn + self.regs.getv(0) as u16 - 2;
        }
    }

    fn op_call(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.stack.push(self.pc);
        self.pc = nnn - 2;
    }

    fn op_se(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
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

    fn op_sne(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
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

    fn op_ld(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
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
                0x55 => {
                    for i in 0..(x+1) as u16 {
                        self.mem.write(self.regs.i + i, self.regs.getv(i as u8));
                    }
                    if self.legacy_ld_sta {
                        self.regs.i += x as u16 + 1;
                    }
                },
                0x65 => { 
                    for i in 0..(x+1) as u16 {
                        self.regs.setv(i as u8, self.mem.read(self.regs.i + i));
                    }
                    if self.legacy_ld_sta {
                        self.regs.i += x as u16 + 1; 
                    }
                },
                _ => { panic!(); }
            }
        }
    }

    fn op_add(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
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

    fn op_or(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) | self.regs.getv(y));
        }
    }

    fn op_xor(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) ^ self.regs.getv(y));
        }
    }

    fn op_and(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if op == 8 {
            self.regs.setv(x, self.regs.getv(x) & self.regs.getv(y));
        }
    }

    fn op_sub(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(x) > self.regs.getv(y) { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(x).wrapping_sub(self.regs.getv(y)));
    }

    fn op_shr(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if self.legacy_shl_shr {
            self.regs.setv(0xF, self.regs.getv(y) & 1);
            self.regs.setv(x, self.regs.getv(y) >> 1);
        }
        else {
            self.regs.setv(0xF, self.regs.getv(x) & 1);
            self.regs.setv(x, self.regs.getv(x) >> 1);
        }
    }

    fn op_subn(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        self.regs.setv(0xF, if self.regs.getv(y) > self.regs.getv(x) { 1 } else { 0 });
        self.regs.setv(x, self.regs.getv(y).wrapping_sub(self.regs.getv(x)));
    }

    fn op_shl(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if self.legacy_shl_shr {
            self.regs.setv(0xF, (self.regs.getv(y) & 0x80) >> 7);
            self.regs.setv(x, self.regs.getv(y) << 1);
        }
        else {
            self.regs.setv(0xF, (self.regs.getv(x) & 0x80) >> 7);
            self.regs.setv(x, self.regs.getv(x) << 1);
        }
    }

    fn op_rnd(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        let rnd: u8 = self.rng.gen();
        self.regs.setv(x, rnd & kk);
    }

    fn op_drw(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        let sprite_height = n as usize;
        let mut sprite = vec![0; sprite_height];
        for i in 0..sprite_height {
            sprite[i] = self.mem.read(self.regs.i + i as u16);
        }
        self.regs.setv(0xF, if self.screen.draw_sprite(self.regs.getv(x), self.regs.getv(y), &sprite) { 1 } else { 0 });
    }

    fn op_skp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (self.input[self.regs.getv(x) as usize]) {
            self.pc += 2;
        }
    }

    fn op_sknp(&mut self, op: u8, nnn: u16, n: u8, x: u8, y: u8, kk: u8) {
        if (!self.input[self.regs.getv(x) as usize]) {
            self.pc += 2;
        }
    }
}