const SCREEN_BUFFER_LENGTH : usize = 64 * 32;

pub struct Screen {
    data: [bool; SCREEN_BUFFER_LENGTH]
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [false; SCREEN_BUFFER_LENGTH]
        }
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        self.data[y as usize * 64 + x as usize]
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, val: bool) {
        self.data[y as usize  * 64 + x as usize ] = val;
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;
                          
        for j in 0..sprite.len() as u8 {
            for i in 0..8 {             
                let curr = ((sprite[j as usize] >> (7-i)) & 1) == 1;
                if curr {
                    let prev = self.get_pixel((x + i) % 64, (y + j) % 32);
                    let new = prev ^ curr;

                    if (prev && !new) {
                        collision = true;                      
                    }

                    self.set_pixel((x + i) % 64, (y + j) % 32, new);
                }
            }
        }
        collision
    }

    pub fn clear_screen(&mut self) {
        for i in 0..SCREEN_BUFFER_LENGTH {
            self.data[i] = false;
        }
    }
}