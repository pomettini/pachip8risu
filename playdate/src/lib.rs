#![no_std]
extern crate pachip8risu;

extern crate alloc;

#[macro_use]
extern crate playdate as pd;
extern crate playdate_controls as controls;

use controls::buttons::PDButtonsExt;
use controls::peripherals::Buttons;
use pachip8risu::Chip8;
use pd::sys::ffi::{LCDColor, LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE};

use core::ptr::NonNull;
use pd::display::Display;
use pd::graphics::bitmap::*;
use pd::graphics::*;
use pd::sys::ffi::PlaydateAPI;
use pd::sys::EventLoopCtrl;
use pd::system::prelude::*;
use pd::system::update::UpdateCtrl;

const WIDTH: i32 = 64;
const HEIGHT: i32 = 32;
const WIDTH_HIRES: i32 = 128;
const HEIGHT_HIRES: i32 = 64;
const SCALE: i32 = 6;
const SCALE_HIRES: i32 = 2;

struct State {
    cpu: Chip8,
}

impl State {
    fn new() -> Self {
        // TODO: Init the state

        let mut cpu = Chip8::new();
        cpu.load_rom(include_bytes!("../../roms/sweetcopter.ch8"), Some(160));

        let ms = System::Cached().seconds_since_epoch();
        cpu.set_random_seed(ms as u64);

        Self { cpu }
    }

    /// System event handler
    fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
        if let SystemEvent::Init = event {
            Display::Default().set_refresh_rate(30.0);

            // Register our update handler that defined below
            self.set_update_handler();
        }
        EventLoopCtrl::Continue
    }
}

impl Update for State {
    /// Updates the state
    fn update(&mut self) -> UpdateCtrl {
        let graphics = Graphics::Cached();

        let buttons = Buttons::Cached();

        if buttons.pushed().a() {
            for i in 0..15 {
                self.cpu.keys[i] = true;
            }
        }

        if buttons.pushed().b() {
            self.cpu.keys[2] = true;
        }

        if buttons.released().a() {
            for i in 0..15 {
                self.cpu.keys[i] = false;
            }
        }

        if buttons.released().b() {
            self.cpu.keys[2] = false;
        }

        match self.cpu.update() {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
            }
        }

        if self.cpu.play_sound() {
            // TODO: Add beep
        }

        let frame_width = 52;
        let scale = 2;
        let lcd_width = 128;
        let lcd_height = 64;

        let frame = graphics.get_frame().unwrap();

        if self.cpu.should_draw {
            for y in 0..lcd_height {
                for x in 0..lcd_width {
                    let pixel = self.cpu.gfx_buffer[y * lcd_width + x];

                    // Draw a 3x3 block for each pixel
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let scaled_x = x * scale + sx;
                            let scaled_y = y * scale + sy;

                            // Calculate the position in the frame
                            let byte_index = (scaled_y * frame_width) + (scaled_x / 8);
                            let bit_index = 7 - (scaled_x % 8); // Bit order is reversed in each byte

                            if pixel {
                                frame[byte_index] |= 1 << bit_index; // Set the pixel
                            } else {
                                frame[byte_index] &= !(1 << bit_index); // Clear the pixel
                            }
                        }
                    }
                }
            }
        }

        graphics.mark_updated_rows(0, 128);

        System::Cached().draw_fps(0, 0);

        UpdateCtrl::Continue
    }
}

#[no_mangle]
pub fn event_handler(
    _api: NonNull<PlaydateAPI>,
    event: SystemEvent,
    _sim_key_code: u32,
) -> EventLoopCtrl {
    // Unsafe static storage for our state.
    // Usually it's safe because there's only one thread.
    pub static mut STATE: Option<State> = None;
    if unsafe { STATE.is_none() } {
        let state = State::new();
        unsafe { STATE = Some(state) }
    }

    // Call state.event
    unsafe { STATE.as_mut().expect("impossible") }.event(event)
}

pub fn draw_row(input: &[bool]) -> u8 {
    input
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc | (!b as u8) << i)
}

// Needed for debug build, absolutely optional
ll_symbols!();
