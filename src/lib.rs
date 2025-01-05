#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;
extern crate playdate_controls as controls;

use api::Cache;
use controls::buttons::PDButtonsExt;
use controls::peripherals::Buttons;
use pd::sys::ffi::{LCD_COLUMNS, LCD_ROWS, LCD_ROWSIZE};

use core::cmp::min;
use core::ops::Add;
use core::ptr::NonNull;
use pd::display::Display;
use pd::graphics::*;
use pd::sys::ffi::PlaydateAPI;
use pd::sys::EventLoopCtrl;
use pd::system::prelude::*;
use pd::system::update::UpdateCtrl;

pub mod pachip8risu;
use pachip8risu::*;

struct State {
    cpu: Chip8,
}

impl State {
    fn new() -> Self {
        // TODO: Init the state

        let mut cpu = Chip8::new();
        cpu.load_rom(include_bytes!("../roms/binding.ch8"), Some(160));

        let ms = System::Cached().seconds_since_epoch();
        cpu.set_random_seed(ms as u64);

        Graphics::Cached().clear_raw(0);

        Self { cpu }
    }

    /// System event handler
    fn event(&'static mut self, event: SystemEvent) -> EventLoopCtrl {
        if let SystemEvent::Init = event {
            Display::Default().set_refresh_rate(50.0);

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
        let system = System::Cached();

        let (up, right, down, left, a, b) = (5, 9, 8, 7, 6, 10);

        if buttons.pushed().up() {
            self.cpu.keys[up] = true;
        }
        if buttons.pushed().right() {
            self.cpu.keys[right] = true;
        }
        if buttons.pushed().down() {
            self.cpu.keys[down] = true;
        }
        if buttons.pushed().left() {
            self.cpu.keys[left] = true;
        }
        if buttons.pushed().a() {
            self.cpu.keys[a] = true;
        }
        if buttons.pushed().b() {
            self.cpu.keys[b] = true;
        }

        if buttons.released().up() {
            self.cpu.keys[up] = false;
        }
        if buttons.released().right() {
            self.cpu.keys[right] = false;
        }
        if buttons.released().down() {
            self.cpu.keys[down] = false;
        }
        if buttons.released().left() {
            self.cpu.keys[left] = false;
        }
        if buttons.released().a() {
            self.cpu.keys[a] = false;
        }
        if buttons.released().b() {
            self.cpu.keys[b] = false;
        }

        let cpu_start = system.seconds_since_epoch_with_ms().1;

        match self.cpu.update() {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
            }
        }

        let cpu_time = system.seconds_since_epoch_with_ms().1 - cpu_start;

        if self.cpu.play_sound() {
            // TODO: Add beep
        }

        let scale = if self.cpu.is_hi_res() { 3 } else { 6 };
        let lcd_width = if self.cpu.is_hi_res() { 128 } else { 64 };
        let lcd_height = if self.cpu.is_hi_res() { 64 } else { 32 };

        let gpu_start = system.seconds_since_epoch_with_ms().1;

        draw(graphics, &mut self.cpu, scale, lcd_width, lcd_height);

        let gpu_time = system.seconds_since_epoch_with_ms().1 - gpu_start;

        /*
        graphics
            .draw_text(format!("CPU: {},  GPU: {}!", cpu_time, gpu_time), 0, 0)
            .unwrap();
        */

        // println!("CPU: {}, GPU: {}!", cpu_time, gpu_time);

        System::Cached().draw_fps(0, 0);

        UpdateCtrl::Continue
    }
}

pub fn draw(graphics: Graphics<Cache>, cpu: &mut Chip8, scale: usize, width: usize, height: usize) {
    let frame = graphics.get_frame().unwrap();

    if cpu.should_draw {
        // Calculate padding to center the framebuffer
        let padding_x = (LCD_COLUMNS as usize - (width * scale)) / 2; // Horizontal padding
        let padding_y = (LCD_ROWS as usize - (height * scale)) / 2; // Vertical padding

        for y in 0..height {
            for x in 0..width {
                let pixel =
                    !cpu.gfx_buffer[(y * width + x + cpu.scroll_x as usize) % cpu.gfx_buffer.len()];

                // Draw a scaled block for each pixel
                for sy in 0..scale {
                    for sx in 0..scale {
                        let scaled_x = padding_x + x * scale + sx; // Add horizontal padding
                        let scaled_y = padding_y + y * scale + sy; // Add vertical padding

                        // Calculate the position in the frame
                        let byte_index = (scaled_y * LCD_ROWSIZE as usize) + (scaled_x / 8);
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

        // Update the rows, including padding offset
        graphics.mark_updated_rows(0, 240);

        /*
        graphics.draw_line(
            0,
            ((cpu.rows_start * scale as u8) + padding_y as u8).into(),
            400,
            ((cpu.rows_start * scale as u8) + padding_y as u8).into(),
            1,
            0,
        );

        graphics.draw_line(
            0,
            ((cpu.rows_end * scale as u8) + padding_y as u8).into(),
            400,
            ((cpu.rows_end * scale as u8) + padding_y as u8).into(),
            1,
            0,
        );
        */

        cpu.reset_rows();
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
