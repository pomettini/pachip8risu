use super::*;

macro_rules! test_opcode {
    ($cpu:expr,$opcode:expr,$entry_point:expr) => {{
        $cpu.memory[$entry_point] = (($opcode & 0xFF00) >> 8) as u8;
        $cpu.memory[$entry_point + 1] = ($opcode & 0x00FF) as u8;
        $cpu.tick();
    }};
}

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

#[test]
fn test_or_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8001, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_and_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8002, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_xor_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8003, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_add_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8004, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_sub_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8005, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_shr_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8006, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_subn_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x8007, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

#[test]
fn test_shl_vx_vy() {
    let mut cpu = Chip8::new();

    test_opcode!(cpu, 0x800E, ENTRY_POINT);

    assert_eq!(cpu.pc, ENTRY_POINT as u16);
}

/*
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
