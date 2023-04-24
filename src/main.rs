extern crate minifb;

mod chip8;

use chip8::Chip8;
use std::{fs::File, io::Read};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut file = File::open("roms/zero.rom").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();

    let mut cpu = Chip8::new();
    cpu.load_rom(&buf);

    let mut window = Window::new(
        "Yet Another Chip-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            transparency: false,
            title: true,
            resize: true,
            scale: Scale::X16,
            scale_mode: ScaleMode::AspectRatioStretch,
            topmost: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.tick();

        /*
        // Ditched, too expensive on the CPU, need to find another
        window.set_title(&format!(
            "Yet Another Chip-8 Emulator - PC: {:#04X} - INDEX: {:#04X} - REG: {:?}",
            &cpu.pc, &cpu.i, &cpu.v
        ));
         */

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(
                &cpu.gfx_buffer.map(|x| u32::from(x) * u32::MAX),
                WIDTH,
                HEIGHT,
            )
            .unwrap();
    }
}
