#![allow(unused)]

extern crate minifb;
use minifb::{Key, KeyRepeat, WindowOptions, Window};

use std::thread;
use std::time::Duration;
use std::time::Instant;

mod emu;
use emu::CPU;

fn main() {
    //Init emulator
    let mut cpu = CPU::new();
    
    let mut unlock_fps = true;
    let mut scale : usize = 6;

    //CLI arguments
    let show_help = || {
        println!("Usage: chipboi <path to file> [Options]");
        println!("Options:");
        println!("  -s <scale>   set framebuffer scale to passed integer");
        println!("  -l           use legacy instruction implementations");
        println!("  -f           lock to 60 FPS");
        println!("  -h           show help menu")
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        show_help();
        return;
    }
    else {
        cpu.load_rom(&args[1]);

        for i in 0..args.len() {
            let arg = &args[i];
            match arg.as_ref() {
                "-h" => show_help(),
                "-f" => unlock_fps = false,
                "-l" => {
                    cpu.legacy_ld_sta = true;
                    cpu.legacy_shl_shr = true;
                },
                "-s" if args.len() > i+1 => scale = args[i+1].parse::<usize>().unwrap(),
                _ => {}
            }
        }
    }

    //Init window
    let width = 64;
    let height = 32;
    let screen_width = width * scale;
    let screen_height = height * scale;
    let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];
    let mut window = Window::new("CHIPBOI - Press U to lock/unlock FPS", screen_width, screen_height, WindowOptions::default()).unwrap();

    //Frame limiting
    let frame_delta : u128 = 1000 / 60;
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
        thread::sleep(Duration::from_millis(1));

        //Check frame dt, 60 hz loop
        let should_update = last_frame_time.elapsed().as_millis() >= frame_delta;

        //Step cpu
        if should_update || unlock_fps {
            cpu.step();
        }

        //Main event loop
        if should_update {
            last_frame_time = Instant::now();

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
            if window.is_key_pressed(Key::U, KeyRepeat::No) {
                unlock_fps = !unlock_fps;
            }

            //Only need to write to display at 60 fps      
            for x in 0..width {
                for y in 0..height {
                    for scr_x in x*scale..(x+1)*scale {
                        for scr_y in y*scale..(y+1)*scale {
                            buffer[scr_y * screen_width + scr_x] = if cpu.screen.get_pixel(x as u8, y as u8) { u32::max_value() } else { 0 };
                        }
                    }
                }
            }
            window.update_with_buffer(&buffer).unwrap();
        }        
    }
}
