extern crate alloc;
extern crate rand;

use core::cmp::max;
use core::cmp::min;

use alloc::boxed::Box;
use alloc::vec;
use anyhow::Error;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const KEYS: usize = 16;
const RAM_SIZE: usize = 65536;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = (SCREEN_WIDTH * 2) * (SCREEN_HEIGHT * 2);
const ENTRY_POINT: usize = 512;
const DEFAULT_TICK_RATE: u16 = 10;
const BIG_FONT_ADDRESS: usize = 0x50;

// Chip-8
const FONT: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F
];

// Super-Chip and variants
const BIG_FONT: [u8; 10 * 16] = [
    0xFF, 0xFF, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, // 0
    0x18, 0x78, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0xFF, 0xFF, // 1
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // 2
    0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 3
    0xC3, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0x03, 0x03, // 4
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 5
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 6
    0xFF, 0xFF, 0x03, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x18, // 7
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, // 8
    0xFF, 0xFF, 0xC3, 0xC3, 0xFF, 0xFF, 0x03, 0x03, 0xFF, 0xFF, // 9
    0x7E, 0xFF, 0xC3, 0xC3, 0xC3, 0xFF, 0xFF, 0xC3, 0xC3, 0xC3, // A
    0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, 0xC3, 0xC3, 0xFC, 0xFC, // B
    0x3C, 0xFF, 0xC3, 0xC0, 0xC0, 0xC0, 0xC0, 0xC3, 0xFF, 0x3C, // C
    0xFC, 0xFE, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xC3, 0xFE, 0xFC, // D
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, // E
    0xFF, 0xFF, 0xC0, 0xC0, 0xFF, 0xFF, 0xC0, 0xC0, 0xC0, 0xC0, // F
];

// #[derive(Debug)]
pub struct Chip8 {
    i: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    v: [u8; REGISTERS],
    pc: u16,
    dt: u8,
    st: u8,
    pub keys: [bool; KEYS],

    // RAM
    memory: Box<[u8]>,
    pub gfx_buffer: Box<[bool]>,

    // Needed for the emulator
    rnd_seed: Option<SmallRng>,
    tick_rate: u16,
    pub should_draw: bool,
    hi_res: bool,

    pub scroll_x: i8,

    pub rows_start: u8,
    pub rows_end: u8,
}

impl Chip8 {
    #[must_use]
    pub fn new() -> Self {
        Self {
            i: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            v: [0; REGISTERS],
            pc: ENTRY_POINT as u16,
            dt: 0,
            st: 0,
            keys: [false; KEYS],
            memory: vec![0; RAM_SIZE].into_boxed_slice(),
            gfx_buffer: vec![false; SCREEN_SIZE].into_boxed_slice(),
            rnd_seed: None,
            tick_rate: DEFAULT_TICK_RATE,
            should_draw: false,
            hi_res: false,
            scroll_x: 0,
            rows_start: 0,
            rows_end: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = ENTRY_POINT as u16;
        self.sp = 0;
        self.memory = vec![0; RAM_SIZE].into_boxed_slice();
        self.gfx_buffer = vec![false; SCREEN_SIZE].into_boxed_slice();
    }

    pub fn load_rom(&mut self, rom_buf: &[u8], tick_rate: Option<u16>) {
        // Load rom at address 0x200
        self.memory[ENTRY_POINT..(rom_buf.len() + ENTRY_POINT)].copy_from_slice(rom_buf);

        // Load font at address 0x000
        self.memory[0..FONT.len()].copy_from_slice(&FONT);

        // Load big font at address 0x050
        self.memory[BIG_FONT_ADDRESS..BIG_FONT_ADDRESS + BIG_FONT.len()].copy_from_slice(&BIG_FONT);

        // Set tick rate
        if let Some(x) = tick_rate {
            self.tick_rate = x;
        }
    }

    pub fn set_random_seed(&mut self, seed: u64) {
        let small_rng = SmallRng::seed_from_u64(seed);
        self.rnd_seed = Some(small_rng);
    }

    #[inline]
    pub const fn get_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16)
    }

    #[inline]
    const fn get_nibbles(&self, opcode: u16) -> (u16, u16, u16, u16) {
        let nib_1 = (opcode & 0xF000) >> 12;
        let nib_2 = (opcode & 0x0F00) >> 8;
        let nib_3 = (opcode & 0x00F0) >> 4;
        let nib_4 = opcode & 0x000F;
        (nib_1, nib_2, nib_3, nib_4)
    }

    #[inline]
    const fn get_variables(&self, opcode: u16) -> (u8, u8, u16, u8, u8) {
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let nnn: u16 = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        (x, y, nnn, kk, n)
    }

    #[inline]
    pub const fn is_hi_res(&self) -> bool {
        self.hi_res
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.should_draw = false;
        self.update_timers();
        for _ in 0..self.tick_rate {
            self.tick()?;
        }
        Ok(())
    }

    fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn full_screen_rows(&mut self) {
        self.rows_start = 0;
        self.rows_end = u8::MAX;
    }

    pub fn reset_rows(&mut self) {
        self.rows_start = u8::MAX;
        self.rows_end = 0;
    }

    #[must_use]
    pub const fn play_sound(&self) -> bool {
        self.st == 1
    }

    #[inline]
    pub const fn width(&self) -> usize {
        if self.is_hi_res() {
            128
        } else {
            64
        }
    }

    #[inline]
    pub const fn height(&self) -> usize {
        if self.is_hi_res() {
            64
        } else {
            32
        }
    }

    // Start opcodes

    fn scd(&mut self, n: u8) {
        self.gfx_buffer.rotate_right(SCREEN_WIDTH * n as usize);
        self.pc += 2;
    }

    fn scu(&mut self, n: u8) {
        self.gfx_buffer.rotate_left(SCREEN_WIDTH * n as usize);
        self.pc += 2;
    }

    /// Clear screen
    fn cls(&mut self) {
        self.gfx_buffer = vec![false; SCREEN_SIZE].into_boxed_slice();
        self.full_screen_rows();
        self.should_draw = true;
        self.pc += 2;
    }

    /// Return from subroutine
    fn ret(&mut self) -> Result<(), Error> {
        if self.sp < 1 {
            return Err(anyhow::anyhow!("Stack underflow"));
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2;
        Ok(())
    }

    fn scr(&mut self) {
        // self.gfx_buffer.rotate_right(4);
        self.scroll_x += 4;
        self.full_screen_rows();
        self.should_draw = true;
        self.pc += 2;
    }

    fn scl(&mut self) {
        // self.gfx_buffer.rotate_left(4);
        self.scroll_x -= 4;
        self.full_screen_rows();
        self.should_draw = true;
        self.pc += 2;
    }

    fn exit(&mut self) {
        panic!("Exit");
    }

    fn low(&mut self) {
        self.hi_res = false;
        self.pc += 2;
    }

    fn high(&mut self) {
        self.hi_res = true;
        self.pc += 2;
    }

    /// Jumps to address NNN
    fn jp_addr(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /// Calls subroutine at NNN
    fn call_addr(&mut self, nnn: u16) -> Result<(), Error> {
        if self.sp >= (size_of::<u16>() * STACK_SIZE) as u8 {
            return Err(anyhow::anyhow!("Stack overflow"));
        }
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
        Ok(())
    }

    /// Skips the next instruction if VX equals NN.
    fn se_vx_byte(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] == kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Skips the next instruction if VX does not equal NN.
    fn sne_vx_byte(&mut self, x: u8, kk: u8) {
        if self.v[x as usize] != kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Skips the next instruction if VX equals VY.
    fn se_vx_vy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Sets VX to NN.
    fn ld_vx_byte(&mut self, x: u8, kk: u8) {
        self.v[x as usize] = kk;
        self.pc += 2;
    }

    /// Adds NN to VX.
    fn add_vx_byte(&mut self, x: u8, kk: u8) {
        self.v[x as usize] += kk;
        self.pc += 2;
    }

    /// Sets VX to the value of VY.
    fn ld_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] = self.v[y as usize];
        self.pc += 2;
    }

    /// Sets VX to (VX OR VY).
    fn or_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
        self.pc += 2;
    }

    /// Sets VX to (VX AND VY).
    fn and_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
        self.pc += 2;
    }

    /// Sets VX to (VX XOR VY).
    fn xor_vx_vy(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
        self.pc += 2;
    }

    ///Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let flag = self.v[y as usize] > (0xFF - self.v[x as usize]);

        self.v[x as usize] += self.v[y as usize];

        if flag {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.pc += 2;
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) {
        // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
        let flag = self.v[y as usize] > self.v[x as usize];

        self.v[x as usize] -= self.v[y as usize];

        if flag {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }

        self.pc += 2;
    }

    /// Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
    fn shr_vx_vy(&mut self, x: u8) {
        let flag = self.v[x as usize] & 0x1;
        self.v[x as usize] >>= 1;
        self.v[0xF] = flag;
        self.pc += 2;
    }

    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    fn subn_vx_vy(&mut self, x: u8, y: u8) {
        let flag = self.v[y as usize] >= self.v[x as usize];

        self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);

        if flag {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.pc += 2;
    }

    /// Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
    fn shl_vx_vy(&mut self, x: u8) {
        let flag = self.v[x as usize] >> 7;
        self.v[x as usize] <<= 1;
        self.v[0xF] = flag;
        self.pc += 2;
    }

    /// Skips the next instruction if VX doesn't equal VY
    fn sne_vx_vy(&mut self, x: u8, y: u8) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Sets I to the address NNN.
    fn ld_i_addr(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc += 2;
    }

    /// Jumps to the address NNN plus V0.
    fn jp_v0_addr(&mut self, nnn: u16) {
        self.pc = nnn + self.v[0] as u16;
        self.pc += 2;
    }

    /// Sets VX to a random number, masked by NN.
    fn rnd_vx_byte(&mut self, x: u8, kk: u8) {
        match self.rnd_seed {
            Some(ref mut seed) => {
                self.v[x as usize] = (seed.next_u32() as u8) & kk;
            }
            None => self.v[x as usize] = 1 & kk,
        }

        self.pc += 2;
    }

    /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
    /// Shamelessly stolen from https://github.com/machinetech/chip8 until I figure out how it works
    fn drw_vx_vy_nibble(&mut self, x: u8, y: u8, n: u8) {
        let gfx_start_x = self.v[x as usize] as usize;
        let gfx_start_y = self.v[y as usize] as usize;

        // Determine sprite dimensions
        let sprite_width = if n == 0 && self.is_hi_res() { 16 } else { 8 };
        let sprite_height = if n == 0 { 16 } else { n as usize };

        self.v[0x0F] = 0; // Clear the collision flag

        for y_offset in 0..sprite_height {
            let sprite_memory_index = self.i as usize + y_offset * (sprite_width / 8);

            // Retrieve the sprite row
            let row_bits = if sprite_width == 16 {
                let high_byte = self.memory[sprite_memory_index] as u16;
                let low_byte = self.memory[sprite_memory_index + 1] as u16;
                (high_byte << 8) | low_byte
            } else {
                self.memory[sprite_memory_index] as u16
            };

            for x_offset in 0..sprite_width {
                let gfx_x = (gfx_start_x + x_offset) % self.width();
                let gfx_y = (gfx_start_y + y_offset) % self.height();
                let pixel_mask = 1 << (sprite_width - 1 - x_offset);
                let sprite_pixel = (row_bits & pixel_mask) != 0;

                if sprite_pixel {
                    let gfx_index = gfx_x + gfx_y * self.width();
                    let was_pixel_set = self.gfx_buffer[gfx_index];
                    self.gfx_buffer[gfx_index] ^= true;

                    if was_pixel_set && !self.gfx_buffer[gfx_index] {
                        self.v[0x0F] = 1; // Collision detected
                    }

                    self.rows_start = min(self.rows_start, gfx_y as u8);
                    self.rows_end = max(self.rows_end, (gfx_y + sprite_height) as u8);

                    self.should_draw = true;
                }
            }
        }

        self.pc += 2; // Increment the program counter
    }

    /// Skips the next instruction if the key stored in VX is pressed.
    fn skp_vx(&mut self, x: u8) {
        if self.keys[self.v[x as usize] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Skips the next instruction if the key stored in VX isn't pressed.
    fn sknp_vx(&mut self, x: u8) {
        if !self.keys[self.v[x as usize] as usize] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    /// Sets VX to the value of the delay timer
    fn ld_vx_dt(&mut self, x: u8) {
        self.v[x as usize] = self.dt;
        self.pc += 2;
    }

    /// A key press is awaited, and then stored in VX
    fn ld_vx_k(&mut self, x: u8) {
        // TODO: Might need refactor
        let mut key_pressed = false;

        for i in 0..KEYS {
            if self.keys[i] {
                self.v[x as usize] = 1;
                key_pressed = true;
            }
        }

        if !key_pressed {
            return;
        }

        self.pc += 2;
    }

    /// Sets the delay timer to VX
    fn ld_dt_vx(&mut self, x: u8) {
        self.dt = self.v[x as usize];
        self.pc += 2;
    }

    /// Sets the sound timer to VX
    fn ld_st_vx(&mut self, x: u8) {
        self.st = self.v[x as usize];
        self.pc += 2;
    }

    /// Adds VX to I
    fn add_i_vx(&mut self, x: u8) {
        self.i += self.v[x as usize] as u16;
        self.pc += 2;
    }

    /// Sets I to the location of the sprite for the character in VX.
    fn ld_f_vx(&mut self, x: u8) {
        self.i = self.v[x as usize] as u16 * 5;
        self.pc += 2;
    }

    fn ld_hf_vx(&mut self, x: u8) {
        self.i = BIG_FONT_ADDRESS as u16 + self.v[x as usize] as u16 * 10;
        self.pc += 2;
    }

    /// Stores the Binary-coded decimal representation of VX at the addresses I, I plus 1, and I plus 2
    fn ld_b_vx(&mut self, x: u8) {
        self.memory[self.i as usize] = self.v[x as usize] / 100;
        self.memory[self.i as usize + 1] = (self.v[x as usize] / 10) % 10;
        self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
        self.pc += 2;
    }

    /// Stores V0 to VX in memory starting at address I
    fn ld_i_vx(&mut self, x: u8) {
        (0..=x).for_each(|i| {
            self.memory[self.i as usize + i as usize] = self.v[i as usize];
        });

        self.i += x as u16 + 1;
        self.pc += 2;
    }

    fn ld_r_vx(&mut self) {
        unimplemented!();
    }

    fn ld_vx_r(&mut self) {
        unimplemented!();
    }

    fn ld_vx_i(&mut self, x: u8) {
        (0..=x).for_each(|i| {
            self.v[i as usize] = self.memory[self.i as usize + i as usize];
        });

        self.i += x as u16 + 1;
        self.pc += 2;
    }

    // End opcodes

    fn tick(&mut self) -> Result<(), Error> {
        let opcode = self.get_opcode();
        let nibbles = self.get_nibbles(opcode);
        let (x, y, nnn, kk, n) = self.get_variables(opcode);

        match nibbles {
            (0, 0, 0xC, _) => self.scd(n),
            (0, 0, 0xD, _) => self.scu(n),
            (0, 0, 0xE, 0) => self.cls(),
            (0, 0, 0xE, 0xE) => self.ret()?,
            (0, 0, 0xF, 0xB) => self.scr(),
            (0, 0, 0xF, 0xC) => self.scl(),
            (0, 0, 0xF, 0xD) => self.exit(),
            (0, 0, 0xF, 0xE) => self.low(),
            (0, 0, 0xF, 0xF) => self.high(),
            (0x1, _, _, _) => self.jp_addr(nnn),
            (0x2, _, _, _) => self.call_addr(nnn)?,
            (0x3, _, _, _) => self.se_vx_byte(x, kk),
            (0x4, _, _, _) => self.sne_vx_byte(x, kk),
            (0x5, _, _, 2) => unimplemented!("Save VX..VY to memory starting at I"),
            (0x5, _, _, 3) => unimplemented!("Load VX..VY from memory starting at I"),
            (0x5, _, _, _) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx_byte(x, kk),
            (0x7, _, _, _) => self.add_vx_byte(x, kk),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or_vx_vy(x, y),
            (0x8, _, _, 0x2) => self.and_vx_vy(x, y),
            (0x8, _, _, 0x3) => self.xor_vx_vy(x, y),
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub_vx_vy(x, y),
            (0x8, _, _, 0x6) => self.shr_vx_vy(x),
            (0x8, _, _, 0x7) => self.subn_vx_vy(x, y),
            (0x8, _, _, 0xE) => self.shl_vx_vy(x),
            (0x9, _, _, _) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i_addr(nnn),
            (0xB, _, _, _) => self.jp_v0_addr(nnn),
            (0xC, _, _, _) => self.rnd_vx_byte(x, kk),
            (0xD, _, _, _) => self.drw_vx_vy_nibble(x, y, n),
            (0xE, _, 0x9, 0xE) => self.skp_vx(x),
            (0xE, _, 0xA, 0x1) => self.sknp_vx(x),
            (0xF, _, 0x0, 0x1) => unimplemented!("Select drawing planes by bitmask"),
            (0xF, _, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, _, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, _, 0x3, 0x0) => self.ld_hf_vx(x),
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, _, 0x5, 0x5) => self.ld_i_vx(x),
            (0xF, _, 0x7, 0x5) => self.ld_r_vx(),
            (0xF, _, 0x8, 0x5) => self.ld_vx_r(),
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(x),
            (0xF, 0x0, 0x0, 0x2) => unimplemented!("Store 16 bytes in audio pattern buffer, starting at I, to be played by the sound buzzer"),
            (0xF, 0x0, 0x0, 0x0) => unimplemented!("Load I with 16-bit address NNNN"),
            (0xF, 0x0, 0x3, 0xA) => unimplemented!("Set the pitch register to the value in VX"),
            (_, _, _, _) => {
                return Err(anyhow::anyhow!(
                    "Unknown opcode: {opcode:#04X} at {0:#04X}",
                    self.pc - ENTRY_POINT as u16,
                ))
            }
        }

        Ok(())
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
