#![allow(unused)]

extern crate piston_window;
use piston_window::*;

mod emu;
use emu::cpu::CPU;

fn main() {
    //Init emulator
    let mut cpu = CPU::new();
    cpu.load_rom("ROMs/minimal_game.ch8");

    //Init window
    let scale : u32 = 6;
    let mut window: PistonWindow = 
        WindowSettings::new("CHIPBOI", [64*scale, 32*scale])
        .exit_on_esc(true)
        .vsync(true)
        .build().unwrap();

    //Keyboard map
    let input_map = [
        Key::X,
        
        Key::D1, Key::D2, Key::D3,
        Key::Q,  Key::W,  Key::E,
        Key::A,  Key::S,  Key::D,

        Key::Z, Key::C, Key::D4, Key::R, Key::F, Key::V
    ];
    
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            //For cpu instruction just go fast (TM). Ok, this is epic code.
            for i in 0..10 {
                cpu.step();
            }

            //Update timers at 60hz
            cpu.update_timers();

            //Draw screen
            window.draw_2d(&e, |c, g| {               
                for x in 0..64 {
                    for y in 0..32 {
                        let color : [f32; 4] = if cpu.screen.get_pixel(x, y) { [0.0, 0.0, 0.0, 1.0] } else { [1.0; 4] };
                        rectangle(color, 
                            [(x as u32 * scale) as f64, (y as u32 * scale) as f64, scale as f64, scale as f64], 
                            c.transform, 
                            g);
                    }
                }
            });
        }

        //Input
        if let Some(Button::Keyboard(button)) = e.press_args() {
            for i in 0..16 {
                if input_map[i] == button {
                    cpu.input[i] = true;
                }
            }
        }
        if let Some(Button::Keyboard(button)) = e.release_args() {
            for i in 0..16 {
                if input_map[i] == button {
                    cpu.input[i] = false;
                }
            }
        }
    }
}
