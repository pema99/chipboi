pub struct Registers {
    v: [u8; 16],
    pub i: u16,
    pub dt: u8,
    pub st: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0
        }
    }

    pub fn getv(&self, index: u8) -> u8 {
        self.v[index as usize]
    }

    pub fn setv(&mut self, index: u8, val: u8) {
        self.v[index as usize] = val;
    }
}

pub struct Stack {
    data: [u16; 16],
    sp: u8
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: [0; 16],
            sp: 0
        }
    }

    pub fn push(&mut self, val: u16) {
        self.data[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.data[self.sp as usize]
    }
}

pub struct Memory {
    data: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 4096],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    } 
    
    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}