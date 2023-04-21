// extern crate sdl2;

mod chip8;

use chip8::Chip8;
use std::{fs::File, io::Read};

/*
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
*/

fn main() {
    let mut file = File::open("roms/ibm-logo.rom").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();

    let mut cpu = Chip8::new();
    cpu.load_rom(&buf);

    for _ in 0..10000 {
        cpu.tick();
    }

    println!("{:?}", &cpu.gfx_buffer);

    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    */
}
