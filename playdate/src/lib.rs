#![no_std]

extern crate alloc;
extern crate pachip8risu;

use crankstart::{geometry::ScreenRect, log_to_console};
use crankstart_sys::PDButtons;
use euclid::{Point2D, Size2D};
use pachip8risu::Chip8;

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
const WIDTH_HIRES: i32 = 128;
const HEIGHT_HIRES: i32 = 64;
const SCALE: i32 = 6;
const SCALE_HIRES: i32 = 3;

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
        cpu.load_rom(include_bytes!("../../roms/scrolling.rom"), Some(10));

        let (_, ms) = System::get().get_seconds_since_epoch().unwrap();
        cpu.set_random_seed(ms as u64);

        Ok(Box::new(Self { cpu }))
    }
}

impl Game for State {
    fn update(&mut self, _playdate: &mut Playdate) -> Result<(), Error> {
        System::get().draw_fps(0, 0)?;

        match self.cpu.update() {
            Ok(()) => {}
            Err(e) => {
                log_to_console!("{}", e);
            }
        }

        // log_to_console!("{0:#04X}", self.cpu.get_opcode());

        let (_, pressed, released) = System::get().get_button_state().unwrap();

        match pressed {
            PDButtons::kButtonA => {
                for i in 0..16 {
                    self.cpu.keys[i] = true;
                }
            }
            _ => (),
        }

        match released {
            PDButtons::kButtonA => {
                for i in 0..16 {
                    self.cpu.keys[i] = false;
                }
            }
            _ => (),
        }

        if !self.cpu.is_hi_res() {
            if let Some(gfx_buffer) = self.cpu.draw() {
                for p in 0..WIDTH * HEIGHT {
                    let x = 8 + (p % 64) * SCALE;
                    let y = 24 + (p / 64) * SCALE;
                    Graphics::get().fill_rect(
                        ScreenRect::new(Point2D::new(x, y), Size2D::new(SCALE, SCALE)),
                        draw_pixel_color(gfx_buffer[p as usize]),
                    )?;
                }
            }
        } else {
            if let Some(gfx_buffer) = self.cpu.draw() {
                for p in 0..WIDTH_HIRES * HEIGHT_HIRES {
                    let x = 8 + (p % 128) * SCALE_HIRES;
                    let y = 24 + (p / 128) * SCALE_HIRES;
                    Graphics::get().fill_rect(
                        ScreenRect::new(Point2D::new(x, y), Size2D::new(SCALE_HIRES, SCALE_HIRES)),
                        draw_pixel_color(gfx_buffer[p as usize]),
                    )?;
                }
            }
        }

        if self.cpu.play_sound() {
            // TODO: Add beep
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
