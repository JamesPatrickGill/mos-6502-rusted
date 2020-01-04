#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::cpu::{Flags, CPU};
use crate::cpu_addrmode;

pub fn ADC(cpu: &mut CPU) -> u8 {
    cpu.fetch();

    cpu.temp = cpu.a as u16 + cpu.fetched as u16 + cpu.get_flag(Flags::C as u8) as u16;

    cpu.set_flag(Flags::C as u8, cpu.temp > 255);

    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);

    cpu.set_flag(
        Flags::V as u8,
        (!((cpu.a as u16) ^ (cpu.fetched as u16)) & ((cpu.a as u16) ^ (cpu.temp as u16)) & 0x0080)
            != 0,
    );

    cpu.set_flag(Flags::N as u8, cpu.temp & 0x80 != 0);

    cpu.a = (cpu.temp & 0x00FF) as u8;

    1
}

pub fn SBC(cpu: &mut CPU) -> u8 {
    cpu.fetch();

    let value: u16 = (cpu.fetched as u16) ^ 0x00FF;

    cpu.temp = (cpu.a as u16) + value + (cpu.get_flag(Flags::C as u8) as u16);
    cpu.set_flag(Flags::C as u8, cpu.temp & 0xFF00 != 0);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(
        Flags::V as u8,
        ((cpu.temp ^ (cpu.a as u16)) & (cpu.temp ^ value) & 0x0080) != 0,
    );
    cpu.set_flag(Flags::N as u8, (cpu.temp & 0x0080) != 0);
    cpu.a = (cpu.temp & 0x00FF) as u8;
    1
}

pub fn AND(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.a &= cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    1
}

pub fn ASL(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.fetched as u16) << 1;
    cpu.set_flag(Flags::C as u8, (cpu.temp & 0xFF00) > 0);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x80 != 0);

    if cpu.lookup[cpu.opcode as usize].addrmode as usize == cpu_addrmode::IMP as usize {
        cpu.a = (cpu.temp & 0x00FF) as u8;
    } else {
        cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    }
    0
}

pub fn BCC(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::C as u8) == 0 {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BCS(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::C as u8) == 1 {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BEQ(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::Z as u8) == 1 {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BIT(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.a & cpu.fetched).into();
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.fetched & (1 << 7) != 0);
    cpu.set_flag(Flags::V as u8, cpu.fetched & (1 << 6) != 0);
    0
}

pub fn BMI(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::N as u8) == 1 {
        cpu.cycles += 1;
        cpu.addr_abs = (cpu.pc).wrapping_add(cpu.addr_rel);

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BNE(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::Z as u8) == 0 {
        cpu.cycles += 1;
        cpu.addr_abs = (cpu.pc).wrapping_add(cpu.addr_rel);

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BPL(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::N as u8) == 0 {
        cpu.cycles += 1;
        cpu.addr_abs = (cpu.pc).wrapping_add(cpu.addr_rel);

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BRK(cpu: &mut CPU) -> u8 {
    cpu.pc += 1;

    cpu.set_flag(Flags::I as u8, true);
    cpu.write_ram(0x0100 + cpu.stkp as u16, (cpu.pc >> 8 & 0x00FF) as u8);
    cpu.stkp -= 1;
    cpu.write_ram(0x0100 + cpu.stkp as u16, (cpu.pc & 0x00FF) as u8);
    cpu.stkp -= 1;

    cpu.set_flag(Flags::B as u8, true);
    cpu.write_ram(0x0100 + cpu.stkp as u16, cpu.status);
    cpu.stkp -= 1;
    cpu.set_flag(Flags::B as u8, false);

    cpu.pc = cpu.read_ram(0xFFFE, false) as u16 | ((cpu.read_ram(0xFFFF, false) as u16) << 8);
    0
}

pub fn BVC(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::V as u8) == 0 {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn BVS(cpu: &mut CPU) -> u8 {
    if cpu.get_flag(Flags::V as u8) == 1 {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn CLC(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::C as u8, false);
    0
}
pub fn CLD(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::D as u8, false);
    0
}
pub fn CLI(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::I as u8, false);
    0
}
pub fn CLV(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::V as u8, false);
    0
}

pub fn CMP(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.a as u16).wrapping_sub(cpu.fetched as u16);
    cpu.set_flag(Flags::C as u8, cpu.a >= cpu.fetched);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    1
}
pub fn CPX(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.x as u16).wrapping_sub(cpu.fetched as u16);
    cpu.set_flag(Flags::C as u8, cpu.x >= cpu.fetched);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    0
}
pub fn CPY(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.y as u16).wrapping_sub(cpu.fetched as u16);
    cpu.set_flag(Flags::C as u8, cpu.y >= cpu.fetched);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    0
}

pub fn DEC(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.fetched - 1) as u16;
    cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    0
}
pub fn DEX(cpu: &mut CPU) -> u8 {
    cpu.x -= 1;
    cpu.set_flag(Flags::Z as u8, cpu.x == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.x & 0x80 != 0);
    0
}
pub fn DEY(cpu: &mut CPU) -> u8 {
    cpu.y -= 1;
    cpu.set_flag(Flags::Z as u8, cpu.y == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.y & 0x80 != 0);
    0
}

pub fn EOR(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.a ^= cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    1
}
pub fn INC(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = (cpu.fetched + 1) as u16;
    cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    0
}
pub fn INX(cpu: &mut CPU) -> u8 {
    cpu.x += 1;
    cpu.set_flag(Flags::Z as u8, cpu.x == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.x & 0x80 != 0);
    0
}
pub fn INY(cpu: &mut CPU) -> u8 {
    cpu.y += 1;
    cpu.set_flag(Flags::Z as u8, cpu.y == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.y & 0x80 != 0);
    0
}

pub fn JMP(cpu: &mut CPU) -> u8 {
    cpu.pc = cpu.addr_abs;
    0
}

pub fn JSR(cpu: &mut CPU) -> u8 {
    cpu.pc -= 1;

    cpu.write_ram(
        0x0100 as u16 + cpu.stkp as u16,
        ((cpu.pc >> 8) & 0x00FF) as u8,
    );
    cpu.stkp -= 1;
    cpu.write_ram(0x0100 + cpu.stkp as u16, (cpu.pc & 0x00FF) as u8);
    cpu.stkp -= 1;

    cpu.pc = cpu.addr_abs;
    0
}

pub fn LDA(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.a = cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    1
}
pub fn LDX(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.x = cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.x == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.x & 0x80 != 0);
    1
}
pub fn LDY(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.y = cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.y == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.y & 0x80 != 0);
    1
}

pub fn LSR(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.set_flag(Flags::C as u8, cpu.fetched & 0x0001 != 0);
    cpu.temp = (cpu.fetched >> 1) as u16;
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);

    if cpu.lookup[cpu.opcode as usize].addrmode as usize == cpu_addrmode::IMP as usize {
        cpu.a = (cpu.temp & 0x00FF) as u8;
    } else {
        cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    }
    0
}

pub fn NOP(cpu: &mut CPU) -> u8 {
    match cpu.opcode {
        0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1,
        _ => 0,
    }
}

pub fn ORA(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.a |= cpu.fetched;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    1
}
pub fn PHA(cpu: &mut CPU) -> u8 {
    cpu.write_ram(0x0100 + cpu.stkp as u16, cpu.a);
    cpu.stkp -= 1;
    0
}
pub fn PHP(cpu: &mut CPU) -> u8 {
    cpu.write_ram(
        0x0100 + cpu.stkp as u16,
        cpu.status | Flags::B as u8 | Flags::U as u8,
    );
    cpu.set_flag(Flags::B as u8, false);
    cpu.set_flag(Flags::U as u8, false);
    cpu.stkp -= 1;
    0
}
pub fn PLA(cpu: &mut CPU) -> u8 {
    cpu.stkp += 1;
    cpu.a = cpu.read_ram(0x0100 + cpu.stkp as u16, false);
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    0
}
pub fn PLP(cpu: &mut CPU) -> u8 {
    cpu.stkp += 1;
    cpu.status = cpu.read_ram(0x0100 + cpu.stkp as u16, false);
    cpu.set_flag(Flags::U as u8, true);
    0
}
pub fn ROL(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = ((cpu.fetched as u16) << 1) | cpu.get_flag(Flags::C as u8) as u16;
    cpu.set_flag(Flags::C as u8, cpu.temp & 0xFF00 != 0);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);

    if cpu.lookup[cpu.opcode as usize].addrmode as usize == cpu_addrmode::IMP as usize {
        cpu.a = (cpu.temp & 0x00FF) as u8;
    } else {
        cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    }
    0
}
pub fn ROR(cpu: &mut CPU) -> u8 {
    cpu.fetch();
    cpu.temp = ((cpu.get_flag(Flags::C as u8) as u16) << 7) | ((cpu.fetched as u16) >> 1);
    cpu.set_flag(Flags::C as u8, cpu.fetched & 0x01 != 0);
    cpu.set_flag(Flags::Z as u8, cpu.temp.trailing_zeros() >= 8);
    cpu.set_flag(Flags::N as u8, cpu.temp & 0x0080 != 0);
    if cpu.lookup[cpu.opcode as usize].addrmode as usize == cpu_addrmode::IMP as usize {
        cpu.a = (cpu.temp & 0x00FF) as u8;
    } else {
        cpu.write_ram(cpu.addr_abs, (cpu.temp & 0x00FF) as u8);
    }
    0
}
pub fn RTI(cpu: &mut CPU) -> u8 {
    cpu.stkp += 1;
    cpu.status = cpu.read_ram(0x0100 + cpu.stkp as u16, false);
    cpu.status &= !(Flags::B as u8);
    cpu.status &= !(Flags::U as u8);

    cpu.stkp += 1;
    cpu.pc = cpu.read_ram(0x0100 + cpu.stkp as u16, false) as u16;
    cpu.stkp += 1;
    cpu.pc |= (cpu.read_ram(0x0100 + cpu.stkp as u16, false) as u16) << 8;
    0
}
pub fn RTS(cpu: &mut CPU) -> u8 {
    cpu.stkp += 1;
    cpu.pc = cpu.read_ram(0x0100 + cpu.stkp as u16, false) as u16;
    cpu.stkp += 1;
    cpu.pc |= (cpu.read_ram(0x0100 + cpu.stkp as u16, false) as u16) << 8;

    cpu.pc += 1;
    0
}

pub fn SEC(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::C as u8, true);
    0
}
pub fn SED(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::D as u8, true);
    0
}
pub fn SEI(cpu: &mut CPU) -> u8 {
    cpu.set_flag(Flags::I as u8, true);
    0
}
pub fn STA(cpu: &mut CPU) -> u8 {
    cpu.write_ram(cpu.addr_abs, cpu.a);
    0
}
pub fn STX(cpu: &mut CPU) -> u8 {
    cpu.write_ram(cpu.addr_abs, cpu.x);
    0
}
pub fn STY(cpu: &mut CPU) -> u8 {
    cpu.write_ram(cpu.addr_abs, cpu.y);
    0
}

pub fn TAX(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.a;
    cpu.set_flag(Flags::Z as u8, cpu.x == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.x & 0x80 != 0);
    0
}
pub fn TAY(cpu: &mut CPU) -> u8 {
    cpu.y = cpu.a;
    cpu.set_flag(Flags::Z as u8, cpu.y == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.y & 0x80 != 0);
    0
}
pub fn TSX(cpu: &mut CPU) -> u8 {
    cpu.x = cpu.stkp;
    cpu.set_flag(Flags::Z as u8, cpu.x == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.x & 0x80 != 0);
    0
}
pub fn TXA(cpu: &mut CPU) -> u8 {
    cpu.a = cpu.x;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    0
}
pub fn TYA(cpu: &mut CPU) -> u8 {
    cpu.a = cpu.y;
    cpu.set_flag(Flags::Z as u8, cpu.a == 0x00);
    cpu.set_flag(Flags::N as u8, cpu.a & 0x80 != 0);
    0
}
pub fn TXS(cpu: &mut CPU) -> u8 {
    cpu.stkp = cpu.x;
    0
}
pub fn XXX(_cpu: &mut CPU) -> u8 {
    0
}
