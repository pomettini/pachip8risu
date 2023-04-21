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
    pub stack: [u16; STACK_SIZE],
    pub memory: [u8; RAM_SIZE],
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
        for i in 0..buf.len() {
            self.memory[i + ENTRY_POINT] = buf[i];
        }
    }

    pub fn tick(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        match opcode & 0xF000 {
            // 00E_
            0x0000 => {
                match opcode & 0x000F {
                    // 00E0 - Clear screen
                    0x0000 => {
                        // TODO: Clear screen
                        self.pc += 2;
                    }
                    // 00EE - Return from subroutine
                    0x000E => {
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    }
                    _ => panic!("Unknown opcode"),
                }
            }

            // 1NNN - Jumps to address NNN
            0x1000 => {
                self.pc = opcode & 0x0FFF;
            }

            // 2NNN - Calls subroutine at NNN
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }

            // 3XNN - Skips the next instruction if VX equals NN.
            0x3000 => {
                if self.v[((opcode & 0x0F00) >> 8) as usize] as u16 == (opcode & 0x00FF) {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // ---

            // ANNN - Sets I to the address NNN.
            0xA000 => {
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }

            // BNNN - Jumps to the address NNN plus V0.
            0xB000 => {
                self.pc = (opcode & 0x0FFF) + self.v[0] as u16;
                self.pc += 2;
            }

            _ => (),
            // _ => panic!("Unknown opcode"),
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

    /*
    #[test]
    fn test_cls() {
        let mut cpu = Chip8::new();
        //cpu.gfx_buffer = [1; SCREEN_WIDTH * SCREEN_HEIGHT];
        //cpu.memory[ENTRY_POINT] = 00E0 as u8;
        cpu.tick();

        assert_eq!(cpu.pc, (ENTRY_POINT + 2) as u16);
        assert_eq!(cpu.gfx_buffer, [0; SCREEN_WIDTH * SCREEN_HEIGHT]);
    }

    #[test]
    fn test_ret() {
        let mut cpu = Chip8::new();
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
