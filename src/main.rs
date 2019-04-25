#![allow(unused)]

extern crate minifb;
use minifb::{Key, WindowOptions, Window};

use std::time::Instant;

mod emu;
use emu::CPU;

fn main() {
    //Init emulator
    let mut cpu = CPU::new();
    cpu.load_rom("ROMs/PONG");

    //Init window
    let scale : usize = 6;
    let width = 64;
    let height = 32;
    let screen_width = width * scale;
    let screen_height = height * scale;
    let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];
    let mut window = Window::new("CHIPBOI", screen_width, screen_height, WindowOptions::default()).unwrap();

    //Frame limiting
    let frame_delta : u128 = 1000 / 1000;
    let mut last_frame_time = Instant::now();

    //Input map
    let input_map : [Key; 16] = [	
        Key::X,
        
        Key::Key1, Key::Key2, Key::Key3,
        Key::Q,  Key::W,  Key::E,
        Key::A,  Key::S,  Key::D,

        Key::Z, Key::C, Key::Key4, Key::R, Key::F, Key::V
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //60 hz loop
        if last_frame_time.elapsed().as_millis() >= frame_delta {
            last_frame_time = Instant::now();

            //Step cpu
            cpu.step();

            //Update dt, st
            cpu.update_timers();  

            //Read input
            for i in 0..16 {
                if window.is_key_down(input_map[i]) {
                    cpu.input[i] = true;
                }
                else {
                    cpu.input[i] = false;
                }
            }

            //Only need to write to display at 60 fps      
            for x in 0..width {
                for y in 0..height {
                    for scr_x in x*scale..(x+1)*scale {
                        for scr_y in y*scale..(y+1)*scale {
                            buffer[scr_y * screen_width + scr_x] = if cpu.screen.get_pixel(x as u8, y as u8) { 4000 } else { 0 };
                        }
                    }
                }
            }
            window.update_with_buffer(&buffer).unwrap();
        }        
    }
}
