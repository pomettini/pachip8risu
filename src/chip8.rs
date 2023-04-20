pub const RAM_SIZE: usize = 4096;

#[derive(Debug, Default)]
pub struct CPU {
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}

impl CPU {
    fn new() -> Self {
        Default::default()
    }

    fn execute(&mut self) {
        // Do stuff based on current program counter

        self.pc += 1;
    }
}

#[derive(Debug)]
pub struct Memory {
    ram: [u8; RAM_SIZE],
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_sys_addr() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }
    */

    #[test]
    fn test_cls() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ret() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_jp_addr() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_call_addr() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_se_vx_byte() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sne_vx_byte() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_se_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_byte() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_vx_byte() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_or_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_and_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_xor_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_shr_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_shl_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_i_addr() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_jp_v0_addr() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_rnd_vx_byte() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_drw_vx_vy_nibble() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_skp_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_sknp_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_k() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_add_i_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_f_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_b_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_i_vx() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }

    #[test]
    fn test_ld_vx_i() {
        let mut cpu = CPU::new();
        cpu.execute();

        assert_eq!(cpu.pc, 100);
    }
}
