#![no_std]

extern crate alloc;
extern crate mchip8;

use crankstart::geometry::ScreenRect;
use euclid::{Point2D, Size2D};
use mchip8::Chip8;

use {
    alloc::boxed::Box,
    anyhow::Error,
    crankstart::{
        crankstart_game,
        graphics::{Graphics, LCDColor, LCDSolidColor},
        system::System,
        Game, Playdate,
    },
};

const WIDTH: i32 = 64;
const HEIGHT: i32 = 32;
const SCALE: i32 = 6;

struct State {
    cpu: Chip8,
}

impl State {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        crankstart::display::Display::get().set_refresh_rate(50.0)?;

        let mut cpu = Chip8::new();
        cpu.load_rom(include_bytes!("../../roms/space-invaders.rom"));

        Ok(Box::new(Self { cpu }))
    }
}

impl Game for State {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        let graphics = Graphics::get();

        System::get().draw_fps(0, 0)?;

        for _ in 0..5 {
            self.cpu.tick();
        }

        if let Some(gfx_buffer) = self.cpu.draw() {
            for p in 0..WIDTH * HEIGHT {
                let x = p % 64;
                let y = p / 64;
                graphics
                    .fill_rect(
                        ScreenRect::new(
                            Point2D::new(x * SCALE, y * SCALE),
                            Size2D::new(SCALE, SCALE),
                        ),
                        LCDColor::Solid(draw_pixel_color(gfx_buffer[p as usize])),
                    )
                    .unwrap();
            }
        }

        Ok(())
    }
}

const fn draw_pixel_color(is_on: bool) -> LCDSolidColor {
    if is_on {
        LCDSolidColor::kColorBlack
    } else {
        LCDSolidColor::kColorWhite
    }
}

crankstart_game!(State);
