#![no_std]

extern crate alloc;
extern crate mchip8;

use crankstart::geometry::ScreenRect;
use crankstart_sys::PDButtons;
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

macro_rules! WHITE {
    () => {
        LCDColor::Solid(LCDSolidColor::kColorWhite)
    };
}

macro_rules! BLACK {
    () => {
        LCDColor::Solid(LCDSolidColor::kColorBlack)
    };
}

impl State {
    pub fn new(_playdate: &Playdate) -> Result<Box<Self>, Error> {
        let mut cpu = Chip8::new();
        cpu.load_rom(include_bytes!("../../roms/breakout.rom"));

        Ok(Box::new(Self { cpu }))
    }
}

impl Game for State {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        System::get().draw_fps(0, 0)?;

        for _ in 0..10 {
            self.cpu.tick();
        }

        let (_, pressed, released) = System::get().get_button_state().unwrap();

        match pressed {
            PDButtons::kButtonLeft => self.cpu.keys[4] = true,
            PDButtons::kButtonRight => self.cpu.keys[6] = true,
            _ => (),
        }

        match released {
            PDButtons::kButtonLeft => self.cpu.keys[4] = false,
            PDButtons::kButtonRight => self.cpu.keys[6] = false,
            _ => (),
        }

        if let Some(gfx_buffer) = self.cpu.draw() {
            for p in 0..WIDTH * HEIGHT {
                let x = 8 + (p % 64) * SCALE;
                let y = 24 + (p / 64) * SCALE;
                Graphics::get()
                    .fill_rect(
                        ScreenRect::new(Point2D::new(x, y), Size2D::new(SCALE, SCALE)),
                        draw_pixel_color(gfx_buffer[p as usize]),
                    )
                    .unwrap();
            }
        }

        Ok(())
    }
}

const fn draw_pixel_color(is_on: bool) -> LCDColor {
    if is_on {
        BLACK!()
    } else {
        WHITE!()
    }
}

crankstart_game!(State);
