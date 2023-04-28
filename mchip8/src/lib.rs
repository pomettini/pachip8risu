#![no_std]

const REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const KEYS: usize = 16;
const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const ENTRY_POINT: usize = 512;
const DEFAULT_TICK_RATE: u16 = 10;

// Chip-8
const FONT: [u8; 80] = [
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
const BIG_FONT: [u8; 160] = [
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

#[derive(Debug)]
pub struct Chip8 {
    pub i: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    pub v: [u8; REGISTERS],
    pub pc: u16,
    pub dt: u8,
    st: u8,
    pub keys: [bool; KEYS],

    // RAM
    pub memory: [u8; RAM_SIZE],
    gfx_buffer: [bool; SCREEN_SIZE],

    // Needed for the emulator
    tick_rate: u16,
    should_draw: bool,
    hi_res: bool,
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
            memory: [0; RAM_SIZE],
            gfx_buffer: [false; SCREEN_SIZE],
            tick_rate: DEFAULT_TICK_RATE,
            should_draw: false,
            hi_res: false,
        }
    }

    pub fn reset(&mut self) {
        self.pc = ENTRY_POINT as u16;
        self.sp = 0;
        self.gfx_buffer = [false; SCREEN_SIZE];
        self.memory = [0; RAM_SIZE];
    }

    pub fn load_rom(&mut self, buf: &[u8], tick_rate: Option<u16>) {
        self.memory[ENTRY_POINT..(buf.len() + ENTRY_POINT)].copy_from_slice(buf);

        // Load font at address 0x000
        self.memory[0..FONT.len()].copy_from_slice(&FONT);

        if let Some(x) = tick_rate {
            self.tick_rate = x
        }
    }

    const fn get_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16)
    }

    pub fn update(&mut self) {
        self.update_timers();
        (0..self.tick_rate).for_each(|_| {
            self.tick();
        });
    }

    fn tick(&mut self) {
        let opcode = self.get_opcode();

        let nib_1 = (opcode & 0xF000) >> 12;
        let nib_2 = (opcode & 0x0F00) >> 8;
        let nib_3 = (opcode & 0x00F0) >> 4;
        let nib_4 = opcode & 0x000F;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        // let n = (opcode & 0x000F) as u8;

        match (nib_1, nib_2, nib_3, nib_4) {
            // 00E0 - Clear screen
            (0, 0, 0xE, 0) => {
                self.gfx_buffer = [false; SCREEN_SIZE];
                self.should_draw = true;
                self.pc += 2;
            }

            // 00EE - Return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            }

            // 1NNN - Jumps to address NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
            }

            // 2NNN - Calls subroutine at NNN
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            // 3XNN - Skips the next instruction if VX equals NN.
            (0x3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 4XNN - Skips the next instruction if VX does not equal NN.
            (0x4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 5XY0 - Skips the next instruction if VX equals VY.
            (0x5, _, _, _) => {
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 6XNN - Sets VX to NN.
            (0x6, _, _, _) => {
                self.v[x] = kk;
                self.pc += 2;
            }

            // 7XNN - Adds NN to VX.
            (0x7, _, _, _) => {
                self.v[x] += kk;
                self.pc += 2;
            }

            // 8XY0 - Sets VX to the value of VY.
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
                self.pc += 2;
            }

            // 8XY1 - Sets VX to (VX OR VY).
            (0x8, _, _, 0x1) => {
                self.v[x] |= self.v[y];
                self.pc += 2;
            }

            // 8XY2 - Sets VX to (VX AND VY).
            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
                self.pc += 2;
            }

            // 8XY3 - Sets VX to (VX XOR VY).
            (0x8, _, _, 0x3) => {
                self.v[x] ^= self.v[y];
                self.pc += 2;
            }

            // 8XY4 - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            (0x8, _, _, 0x4) => {
                self.v[x] += self.v[y];

                if self.v[y] > (0xFF - self.v[x]) {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.pc += 2;
            }

            // 8XY5 - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0x5) => {
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }

                self.v[x] -= self.v[y];
                self.pc += 2;
            }

            // 0x8XY6 - Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
                self.pc += 2;
            }

            // 0x8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            (0x8, _, _, 0x7) => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }

                self.v[x] = self.v[y] - self.v[x];
                self.pc += 2;
            }

            // 0x8XYE: Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] >> 7;
                self.v[x] <<= 1;
                self.pc += 2;
            }

            // 9XY0 - Skips the next instruction if VX doesn't equal VY
            (0x9, _, _, _) => {
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // ANNN - Sets I to the address NNN.
            (0xA, _, _, _) => {
                self.i = nnn;
                self.pc += 2;
            }

            // BNNN - Jumps to the address NNN plus V0.
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;
                self.pc += 2;
            }

            // CXNN - Sets VX to a random number, masked by NN.
            (0xC, _, _, _) => {
                // TODO: Needs a random number
                // self.v[x] = (RAND % 0xFF + 1) & kk;
                self.v[x] = 1 & kk;
                self.pc += 2;
            }

            // DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            (0xD, _, _, _) => {
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let height = opcode & 0x000F;
                self.v[0xF] &= 0;

                // TODO: Needs refactor
                (0..height).for_each(|y| {
                    let pixel = self.memory[(self.i + y) as usize];
                    (0..8).for_each(|x| {
                        if pixel & (0x80 >> x) > 0 {
                            let index = (x + vx + (y + vy) * SCREEN_WIDTH as u16)
                                .clamp(0, SCREEN_SIZE as u16 - 1)
                                as usize;
                            if self.gfx_buffer[index] {
                                self.v[0xF] = 1;
                            }
                            self.gfx_buffer[index] ^= true;
                        }
                    });
                });

                self.should_draw = true;
                self.pc += 2;
            }

            // EX9E - Skips the next instruction if the key stored in VX is pressed.
            (0xE, _, 0x9, 0xE) => {
                if self.keys[self.v[x] as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // EXA1 - Skips the next instruction if the key stored in VX isn't pressed.
            (0xE, _, 0xA, 0x1) => {
                if !self.keys[self.v[x] as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // FX07 - Sets VX to the value of the delay timer
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.dt;
                self.pc += 2;
            }

            // FX0A - A key press is awaited, and then stored in VX
            (0xF, _, 0x0, 0xA) => {
                let mut key_pressed = false;

                for i in 0..KEYS {
                    if self.keys[i] {
                        self.v[x] = 1;
                        key_pressed = true;
                    }
                }

                if !key_pressed {
                    return;
                }

                self.pc += 2;
            }

            // FX15 - Sets the delay timer to VX
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.v[x];
                self.pc += 2;
            }

            // FX18 - Sets the sound timer to VX
            (0xF, _, 0x1, 0x8) => {
                self.st = self.v[x];
                self.pc += 2;
            }

            // FX1E - Adds VX to I
            (0xF, _, 0x1, 0xE) => {
                // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't.
                if (self.i + self.v[x] as u16) > 0xFFF {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }

                self.i += self.v[x] as u16;
                self.pc += 2;
            }

            // FX29 - Sets I to the location of the sprite for the character in VX.
            (0xF, _, 0x2, 0x9) => {
                self.i = self.v[x] as u16 * 0x5;
                self.pc += 2;
            }

            // FX33 - Stores the Binary-coded decimal representation of VX at the addresses I, I plus 1, and I plus 2
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.memory[self.i as usize + 2] = self.v[x] % 10;
                self.pc += 2;
            }

            // FX55 - Stores V0 to VX in memory starting at address I
            (0xF, _, 0x5, 0x5) => {
                (0..=x).for_each(|i| {
                    self.memory[self.i as usize + i] = self.v[i];
                });

                self.i += x as u16 + 1;
                self.pc += 2;
            }

            // FX65
            (0xF, _, 0x6, 0x5) => {
                (0..=x).for_each(|i| {
                    self.v[i] = self.memory[self.i as usize + i];
                });

                self.i += x as u16 + 1;
                self.pc += 2;
            }

            (_, _, _, _) => panic!("Unknown opcode: {opcode:#04X}"),
        }
    }

    pub fn draw(&mut self) -> Option<[bool; SCREEN_WIDTH * SCREEN_HEIGHT]> {
        if self.should_draw {
            self.should_draw = false;
            Some(self.gfx_buffer)
        } else {
            None
        }
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // TODO: Insert real beep here
                // dbg!("BEEP!");
            }
            self.st -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_opcode {
        ($cpu:expr,$opcode:expr,$entry_point:expr) => {{
            $cpu.memory[$entry_point] = (($opcode & 0xFF00) >> 8) as u8;
            $cpu.memory[$entry_point + 1] = ($opcode & 0x00FF) as u8;
            $cpu.tick();
        }};
    }

    /*
    #[test]
    fn test_sys_addr() {}
    */

    #[test]
    fn test_cls() {
        let mut cpu = Chip8::new();

        cpu.gfx_buffer = [true; SCREEN_WIDTH * SCREEN_HEIGHT];

        // CLS
        test_opcode!(cpu, 0x00E0, ENTRY_POINT);

        assert_eq!(cpu.pc, (ENTRY_POINT + 2) as u16);
        assert_eq!(cpu.gfx_buffer, [false; SCREEN_WIDTH * SCREEN_HEIGHT]);
    }

    #[test]
    fn test_ret() {
        let mut cpu = Chip8::new();

        // JMP 0x0ABC
        test_opcode!(cpu, 0x2ABC, ENTRY_POINT);

        // RET
        test_opcode!(cpu, 0x00EE, 0x0ABC);

        assert_eq!(cpu.pc, (ENTRY_POINT + 2) as u16);
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    fn test_jp_addr() {
        let mut cpu = Chip8::new();

        // JMP 0x0A2A
        test_opcode!(cpu, 0x1A2A, ENTRY_POINT);

        assert_eq!(cpu.pc, 0x0A2A);
    }

    #[test]
    fn test_call_addr() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x2ABC, ENTRY_POINT);

        assert_eq!(cpu.pc, 0x0ABC);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.stack[0], ENTRY_POINT as u16);
    }

    #[test]
    fn test_se_vx_byte() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x3000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    #[test]
    fn test_sne_vx_byte() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x4000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    #[test]
    fn test_se_vx_vy() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x5000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    #[test]
    fn test_ld_vx_byte() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x6000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    #[test]
    fn test_add_vx_byte() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x7000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = Chip8::new();

        test_opcode!(cpu, 0x8000, ENTRY_POINT);

        assert_eq!(cpu.pc, ENTRY_POINT as u16);
    }

    /*
    #[test]
    fn test_or_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_and_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_xor_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_shr_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_shl_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_i_addr() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_jp_v0_addr() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_rnd_vx_byte() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_drw_vx_vy_nibble() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_skp_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sknp_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_k() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_i_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_f_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_b_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_i_vx() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_i() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }
    */
}
