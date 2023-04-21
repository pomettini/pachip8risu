mod chip8;

use chip8::Chip8;
use std::{fs::File, io::Read, path::Path};

fn main() {
    let mut file = File::open("roms/maze.rom").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();

    let mut cpu = Chip8::new();
    cpu.load_rom(&buf);

    for _ in 0..10 {
        cpu.tick();
    }
}
