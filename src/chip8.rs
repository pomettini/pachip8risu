extern crate rand;

pub const REGISTERS: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const RAM_SIZE: usize = 4096;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const ENTRY_POINT: usize = 512;

#[derive(Debug)]
pub struct Chip8 {
    pub i: u16,
    pub sp: u8,
    stack: [u16; STACK_SIZE],
    memory: [u8; RAM_SIZE],
    pub v: [u8; REGISTERS],
    pub pc: u16,
    pub dt: u8,
    pub gfx_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            i: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            memory: [0; RAM_SIZE],
            v: [0; REGISTERS],
            pc: ENTRY_POINT as u16,
            dt: 0,
            gfx_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn load_rom(&mut self, buf: &[u8]) {
        self.memory[ENTRY_POINT..(buf.len() + ENTRY_POINT)].copy_from_slice(buf);
    }

    pub fn tick(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        let nib_1 = (opcode & 0xF000) >> 12;
        let nib_2 = (opcode & 0x0F00) >> 8;
        let nib_3 = (opcode & 0x00F0) >> 4;
        let nib_4 = opcode & 0x000F;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        match (nib_1, nib_2, nib_3, nib_4) {
            // 00E0 - Clear screen
            (0, 0, 0xE, 0) => {
                self.gfx_buffer = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
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
                if self.v[x] as u16 == (opcode & 0x00FF) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 4XNN - Skips the next instruction if VX does not equal NN.
            (0x4, _, _, _) => {
                todo!()
            }

            // 5XY0 - Skips the next instruction if VX equals VY.
            (0x5, _, _, _) => {
                todo!()
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

            // 8XY_
            (0x8, _, _, 0x0) => {
                todo!()
            }

            // 9XY0 - Skips the next instruction if VX doesn't equal VY
            (0x9, _, _, _) => {
                todo!()
            }

            // ANNN - Sets I to the address NNN.
            (0xA, _, _, _) => {
                self.i = nnn;
                self.pc += 2;
            }

            // BNNN - Jumps to the address NNN plus V0.
            (0xB, _, _, _) => {
                self.pc = (nnn) + self.v[0] as u16;
                self.pc += 2;
            }

            // CXNN - Sets VX to a random number, masked by NN.
            (0xC, _, _, _) => {
                // TODO: Needs to be tested
                self.v[x] = (rand::random::<u8>() % 0xFF + 1) & kk;
                self.pc += 2;
            }

            // DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            (0xD, _, _, _) => {
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let height = opcode & 0x000F;
                self.v[0xF] &= 0;

                for y in 0..height {
                    let pixel = self.memory[(self.i + y) as usize];
                    for x in 0..8 {
                        if pixel & (0x80 >> x) >= 1 {
                            if self.gfx_buffer[(x + vx + (y + vy) * 64) as usize] >= 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx_buffer[(x + vx + (y + vy) * 64) as usize] ^= 1;
                        }
                    }
                }

                self.pc += 2;
            }

            // EX__
            (0xE, _, 0x9, 0xE) => {
                todo!()
            }

            // FX07 - Sets VX to the value of the delay timer
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.dt;
                self.pc += 2;
            }

            // FX0A - A key press is awaited, and then stored in VX
            (0xF, _, 0x0, 0xA) => {
                todo!()
            }

            // FX15 - Sets the delay timer to VX
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.v[x];
                self.pc += 2;
            }

            // FX1E - Adds VX to I
            (0xF, _, 0x1, 0xE) => {
                todo!()
            }

            // FX29 - Sets I to the location of the sprite for the character in VX.
            (0xF, _, 0x2, 0x9) => {
                self.i = self.v[x] as u16 * 0x5;
                self.pc += 2;
            }

            // FX33 - Stores the Binary-coded decimal representation of VX at the addresses I, I plus 1, and I plus 2
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[(self.i + 1) as usize] = (self.v[x] / 10) % 10;
                self.memory[(self.i + 2) as usize] = self.v[x] % 10;
                self.pc += 2;
            }

            // FX55 - Stores V0 to VX in memory starting at address I
            (0xF, _, 0x5, 0x5) => {
                todo!()
            }

            (0xF, _, 0x6, 0x5) => {
                // TODO: Must be tested
                for i in 0..((opcode & 0x0F00) >> 8) {
                    self.v[i as usize] = self.memory[(self.i + i) as usize];
                }

                self.i += ((opcode & 0x0F00) >> 8) + 1;
                self.pc += 2;
            }

            (_, _, _, _) => panic!("Unknown opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_sys_addr() {}
    */

    #[test]
    fn test_cls() {
        let mut cpu = Chip8::new();

        // Fill the graphics buffer
        cpu.gfx_buffer = [1; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Load CLS
        cpu.memory[ENTRY_POINT] = 00E0 as u8;

        cpu.tick();

        assert_eq!(cpu.pc, (ENTRY_POINT + 2) as u16);
        assert_eq!(cpu.gfx_buffer, [0; SCREEN_WIDTH * SCREEN_HEIGHT]);
    }

    /*
    #[test]
    fn test_ret() {
        let mut cpu = Chip8::new();

        // Load RET
        cpu.memory[ENTRY_POINT] = 00EE as u8;

        cpu.tick();

        assert_eq!(cpu.pc, (ENTRY_POINT + 2) as u16);
    }

    #[test]
    fn test_jp_addr() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_call_addr() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_se_vx_byte() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sne_vx_byte() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_se_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_byte() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_vx_byte() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = Chip8::new();
        cpu.tick();

        assert_eq!(cpu.pc, 100);
    }

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
