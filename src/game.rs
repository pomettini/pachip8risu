use alloc::boxed::Box;
use playdate_menu::api::Api;

use super::*;

pub struct MyGame {
    pub on_state_change: Option<Box<dyn FnMut(MyState)>>,
    cpu: Chip8,
    first: Option<SimpleMenuItem<u32>>,
}

impl Game for MyGame {
    fn new(_: &Playdate) -> Self {
        let cpu = Chip8::new();

        Self {
            on_state_change: None,
            cpu,
            first: None,
        }
    }

    /// Updates the state
    fn update(&mut self, _: &Playdate) {
        let mut should_remove = false;

        if let Some(ref item) = self.first {
            if let Some(userdata) = item.get_userdata() {
                if *userdata == 1 {
                    item.set_userdata(0);
                    if let Some(ref mut callback) = self.on_state_change {
                        callback(MyState::Menu);
                        should_remove = true;
                    }
                }
            }
        }

        if should_remove {
            self.first = None;
        }

        #[cfg(feature = "debug-print-opcode")]
        println!("{0:#04X}", self.cpu.get_opcode());

        let graphics = Graphics::Cached();
        #[cfg(feature = "debug-profile")]
        let system = System::Cached();

        #[cfg(feature = "debug-profile")]
        let cpu_start = system.seconds_since_epoch_with_ms().1;

        handle_inputs(&mut self.cpu);

        match self.cpu.update() {
            Ok(()) => {}
            Err(e) => {
                println!("{}", e);
            }
        }

        #[cfg(feature = "debug-profile")]
        let cpu_time = system.seconds_since_epoch_with_ms().1 - cpu_start;

        if self.cpu.play_sound() {
            // TODO: Add beep
        }

        let scale = if self.cpu.is_hi_res() { 3 } else { 6 };
        let lcd_width = if self.cpu.is_hi_res() { 128 } else { 64 };
        let lcd_height = if self.cpu.is_hi_res() { 64 } else { 32 };

        #[cfg(feature = "debug-profile")]
        let gpu_start = system.seconds_since_epoch_with_ms().1;

        draw(graphics, &mut self.cpu, scale, lcd_width, lcd_height);

        #[cfg(feature = "debug-profile")]
        let gpu_time = system.seconds_since_epoch_with_ms().1 - gpu_start;

        #[cfg(feature = "debug-profile")]
        graphics
            .draw_text(format!("CPU: {},  GPU: {}!", cpu_time, gpu_time), 0, 0)
            .unwrap();

        #[cfg(feature = "debug-profile")]
        println!("CPU: {}, GPU: {}!", cpu_time, gpu_time);

        System::Cached().draw_fps(0, 0);
    }
}

pub fn handle_inputs(cpu: &mut Chip8) {
    let buttons = Buttons::Cached();

    let (up, right, down, left, a, b) = (5, 9, 8, 7, 6, 10);

    // TODO: Needs refactor

    if buttons.pushed().up() {
        cpu.keys[up] = true;
    }
    if buttons.pushed().right() {
        cpu.keys[right] = true;
    }
    if buttons.pushed().down() {
        cpu.keys[down] = true;
    }
    if buttons.pushed().left() {
        cpu.keys[left] = true;
    }
    if buttons.pushed().a() {
        cpu.keys[a] = true;
    }
    if buttons.pushed().b() {
        cpu.keys[b] = true;
    }

    if buttons.released().up() {
        cpu.keys[up] = false;
    }
    if buttons.released().right() {
        cpu.keys[right] = false;
    }
    if buttons.released().down() {
        cpu.keys[down] = false;
    }
    if buttons.released().left() {
        cpu.keys[left] = false;
    }
    if buttons.released().a() {
        cpu.keys[a] = false;
    }
    if buttons.released().b() {
        cpu.keys[b] = false;
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

        #[cfg(feature = "debug-gfx")]
        graphics.draw_line(
            0,
            ((cpu.rows_start * scale as u8) + padding_y as u8).into(),
            400,
            ((cpu.rows_start * scale as u8) + padding_y as u8).into(),
            1,
            0,
        );

        #[cfg(feature = "debug-gfx")]
        graphics.draw_line(
            0,
            ((cpu.rows_end * scale as u8) + padding_y as u8).into(),
            400,
            ((cpu.rows_end * scale as u8) + padding_y as u8).into(),
            1,
            0,
        );

        cpu.reset_rows();
    }
}

impl MyGame {
    pub fn on_enter(&mut self, id: u8) {
        println!("Entered Game state");

        self.cpu
            .load_rom(include_bytes!("../roms/sweetcopter.ch8"), Some(200));

        let ms = System::Cached().seconds_since_epoch();
        self.cpu.set_random_seed(ms as u64);

        let on_change = move |userdata: &mut u32| {
            *userdata = 1;
        };

        self.first = SimpleMenuItem::new("Check Me", Some(on_change), 0)
            .unwrap()
            .into();
    }

    pub fn set_on_state_change<F>(&mut self, callback: F)
    where
        F: FnMut(MyState) + 'static,
    {
        self.on_state_change = Some(Box::new(callback));
    }
}
