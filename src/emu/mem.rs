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
        let mut result = Memory {
            data: [0; 4096],
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
            result.write(hex_char.0 as u16, *hex_char.1);
        }

        result
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    } 
    
    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}