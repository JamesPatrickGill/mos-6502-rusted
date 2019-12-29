#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::cpu::CPU;

pub fn IMP(cpu: &mut CPU) -> u8 {
    cpu.fetched = cpu.a;
    0
}

pub fn IMM(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = cpu.pc;
    cpu.pc += 1;
    0
}

pub fn ZP0(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}

pub fn ZPX(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = (cpu.read_ram(cpu.pc, false) + cpu.x).into();
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}
pub fn ZPY(cpu: &mut CPU) -> u8 {
    cpu.addr_abs = (cpu.read_ram(cpu.pc, false) + cpu.y).into();
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}
pub fn REL(cpu: &mut CPU) -> u8 {
    cpu.addr_rel = cpu.read_ram(cpu.pc, false) as u16;
    cpu.pc += 1;
    if cpu.addr_rel & 0b1000_0000 != 0 {
        cpu.addr_rel |= 0xFF00;
    }
    0
}
pub fn ABS(cpu: &mut CPU) -> u8 {
    let lo: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    let hi: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    cpu.addr_abs = (hi << 8) | lo;
    0
}
pub fn ABX(cpu: &mut CPU) -> u8 {
    let lo: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    let hi: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    cpu.addr_abs = (hi << 8) | lo;
    cpu.addr_abs += cpu.x as u16;

    if (cpu.addr_abs & 0xFF00) != (hi << 8) {
        1
    } else {
        0
    }
}
pub fn ABY(cpu: &mut CPU) -> u8 {
    let lo: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    let hi: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;

    cpu.addr_abs = (hi << 8) | lo;
    cpu.addr_abs += cpu.y as u16;

    if (cpu.addr_abs & 0xFF00) != (hi << 8) {
        1
    } else {
        0
    }
}

pub fn IND(cpu: &mut CPU) -> u8 {
    let ptr_lo: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;
    let ptr_hi: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;

    let ptr: u16 = (ptr_hi << 8) | ptr_lo;

    if ptr_lo == 0x00FF {
        cpu.addr_abs =
            (cpu.read_ram(ptr & 0xFF00, false) as u16) << 8 | cpu.read_ram(ptr, false) as u16;
    } else {
        cpu.addr_abs = (cpu.read_ram(ptr + 1, false) as u16) << 8 | cpu.read_ram(ptr, false) as u16;
    }
    0
}

pub fn IZX(cpu: &mut CPU) -> u8 {
    let t: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;

    let lo: u16 = cpu
        .read_ram((t + cpu.x as u16) as u16 & 0x00FF, false)
        .into();
    let hi: u16 = cpu
        .read_ram((t + cpu.x as u16 + 1) as u16 & 0x00FF, false)
        .into();

    cpu.addr_abs = (hi << 8) | lo;

    0
}

pub fn IZY(cpu: &mut CPU) -> u8 {
    let t: u16 = cpu.read_ram(cpu.pc, false).into();
    cpu.pc += 1;

    let lo: u16 = cpu.read_ram(t & 0x00FF, false).into();
    let hi: u16 = cpu.read_ram((t + 1) & 0x00FF, false).into();

    cpu.addr_abs = (hi << 8) | lo;
    cpu.addr_abs += cpu.y as u16;

    if (cpu.addr_abs & 0xFF00) != (hi << 8) {
        1
    } else {
        0
    }
}
