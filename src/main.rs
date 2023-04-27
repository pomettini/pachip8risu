extern crate mchip8;
extern crate minifb;

use mchip8::Chip8;
use std::{fs::File, io::Read};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let mut file = File::open("roms/breakout.rom").unwrap();
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

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.tick();

        /*
        window.set_title(&format!(
            "Yet Another Chip-8 Emulator - {:#04X} - PC: {:#04X} - INDEX: {:#04X} - REG: {:?}",
            &cpu.get_opcode(),
            &cpu.pc,
            &cpu.i,
            &cpu.v
        ));
        */

        if let Some(gfx_buffer) = cpu.draw() {
            window
                .update_with_buffer(&gfx_buffer.map(|x| u32::from(x) * u32::MAX), WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
