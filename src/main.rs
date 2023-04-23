extern crate minifb;

mod chip8;

use chip8::Chip8;
use std::{fs::File, io::Read};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut file = File::open("roms/ibm-logo.rom").unwrap();
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

    let mut buffer;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.tick();

        buffer = cpu.gfx_buffer.map(|x| (x as u32) * u32::MAX);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
