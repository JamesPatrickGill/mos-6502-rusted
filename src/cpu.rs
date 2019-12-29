#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::cpu_addrmode;
use crate::cpu_ops;

#[repr(u8)]
pub enum Flags {
    C = 1,
    Z = (1 << 1),
    I = (1 << 2),
    D = (1 << 3),
    B = (1 << 4),
    U = (1 << 5),
    V = (1 << 6),
    N = (1 << 7),
}

pub struct INSTRUCTION {
    pub name: &'static str,
    pub operate: fn(&mut CPU) -> u8,
    pub addrmode: fn(&mut CPU) -> u8,
    pub cycles: u8,
}

pub struct CPU {
    // Actual used values
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub stkp: u8,
    pub pc: u16,
    pub status: u8,

    // Useful for emu
    pub fetched: u8,   
    pub temp: u16,     
    pub addr_abs: u16, 
    pub addr_rel: u16, 
    pub opcode: u8,    
    pub cycles: u8,    
    pub clock_count: u32,

    // Connections
    pub bus:  crate::bus::BUS,
    // Opcode Lookup
    pub lookup: [INSTRUCTION; 256],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            // Actuals
            a: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: 0x00,
            // Convenience stuff
            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0,
            clock_count: 0,
            // Connections
            bus: crate::bus::BUS::new(),
            lookup: [INSTRUCTION { name: "BRK", operate: cpu_ops::BRK, addrmode: cpu_addrmode::IMM, cycles: 7 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "ASL", operate: cpu_ops::ASL, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "PHP", operate: cpu_ops::PHP, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "ASL", operate: cpu_ops::ASL, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "ASL", operate: cpu_ops::ASL, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BPL", operate: cpu_ops::BPL, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "ASL", operate: cpu_ops::ASL, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "CLC", operate: cpu_ops::CLC, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ORA", operate: cpu_ops::ORA, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "ASL", operate: cpu_ops::ASL, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "JSR", operate: cpu_ops::JSR, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "BIT", operate: cpu_ops::BIT, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "ROL", operate: cpu_ops::ROL, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "PLP", operate: cpu_ops::PLP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "ROL", operate: cpu_ops::ROL, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "BIT", operate: cpu_ops::BIT, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "ROL", operate: cpu_ops::ROL, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BMI", operate: cpu_ops::BMI, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "ROL", operate: cpu_ops::ROL, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "SEC", operate: cpu_ops::SEC, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "AND", operate: cpu_ops::AND, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "ROL", operate: cpu_ops::ROL, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "RTI", operate: cpu_ops::RTI, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "LSR", operate: cpu_ops::LSR, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "PHA", operate: cpu_ops::PHA, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "LSR", operate: cpu_ops::LSR, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "JMP", operate: cpu_ops::JMP, addrmode: cpu_addrmode::ABS, cycles: 3 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "LSR", operate: cpu_ops::LSR, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BVC", operate: cpu_ops::BVC, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "LSR", operate: cpu_ops::LSR, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "CLI", operate: cpu_ops::CLI, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "EOR", operate: cpu_ops::EOR, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "LSR", operate: cpu_ops::LSR, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "RTS", operate: cpu_ops::RTS, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "ROR", operate: cpu_ops::ROR, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "PLA", operate: cpu_ops::PLA, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "ROR", operate: cpu_ops::ROR, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "JMP", operate: cpu_ops::JMP, addrmode: cpu_addrmode::IND, cycles: 5 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "ROR", operate: cpu_ops::ROR, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BVS", operate: cpu_ops::BVS, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "ROR", operate: cpu_ops::ROR, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "SEI", operate: cpu_ops::SEI, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "ADC", operate: cpu_ops::ADC, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "ROR", operate: cpu_ops::ROR, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "STY", operate: cpu_ops::STY, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "STX", operate: cpu_ops::STX, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "DEY", operate: cpu_ops::DEY, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "TXA", operate: cpu_ops::TXA, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "STY", operate: cpu_ops::STY, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "STX", operate: cpu_ops::STX, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "BCC", operate: cpu_ops::BCC, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::IZY, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "STY", operate: cpu_ops::STY, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "STX", operate: cpu_ops::STX, addrmode: cpu_addrmode::ZPY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "TYA", operate: cpu_ops::TYA, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::ABY, cycles: 5 },
                INSTRUCTION { name: "TXS", operate: cpu_ops::TXS, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "STA", operate: cpu_ops::STA, addrmode: cpu_addrmode::ABX, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "LDY", operate: cpu_ops::LDY, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "LDX", operate: cpu_ops::LDX, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "LDY", operate: cpu_ops::LDY, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "LDX", operate: cpu_ops::LDX, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 3 },
                INSTRUCTION { name: "TAY", operate: cpu_ops::TAY, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "TAX", operate: cpu_ops::TAX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "LDY", operate: cpu_ops::LDY, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "LDX", operate: cpu_ops::LDX, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "BCS", operate: cpu_ops::BCS, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "LDY", operate: cpu_ops::LDY, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "LDX", operate: cpu_ops::LDX, addrmode: cpu_addrmode::ZPY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "CLV", operate: cpu_ops::CLV, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "TSX", operate: cpu_ops::TSX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "LDY", operate: cpu_ops::LDY, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "LDA", operate: cpu_ops::LDA, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "LDX", operate: cpu_ops::LDX, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "CPY", operate: cpu_ops::CPY, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "CPY", operate: cpu_ops::CPY, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "DEC", operate: cpu_ops::DEC, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "INY", operate: cpu_ops::INY, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "DEX", operate: cpu_ops::DEX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "CPY", operate: cpu_ops::CPY, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "DEC", operate: cpu_ops::DEC, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BNE", operate: cpu_ops::BNE, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "DEC", operate: cpu_ops::DEC, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "CLD", operate: cpu_ops::CLD, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "NOP", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "CMP", operate: cpu_ops::CMP, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "DEC", operate: cpu_ops::DEC, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "CPX", operate: cpu_ops::CPX, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::IZX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "CPX", operate: cpu_ops::CPX, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::ZP0, cycles: 3 },
                INSTRUCTION { name: "INC", operate: cpu_ops::INC, addrmode: cpu_addrmode::ZP0, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 5 },
                INSTRUCTION { name: "INX", operate: cpu_ops::INX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::IMM, cycles: 2 },
                INSTRUCTION { name: "NOP", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::SBC, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "CPX", operate: cpu_ops::CPX, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::ABS, cycles: 4 },
                INSTRUCTION { name: "INC", operate: cpu_ops::INC, addrmode: cpu_addrmode::ABS, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "BEQ", operate: cpu_ops::BEQ, addrmode: cpu_addrmode::REL, cycles: 2 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::IZY, cycles: 5 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 8 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::ZPX, cycles: 4 },
                INSTRUCTION { name: "INC", operate: cpu_ops::INC, addrmode: cpu_addrmode::ZPX, cycles: 6 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 6 },
                INSTRUCTION { name: "SED", operate: cpu_ops::SED, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::ABY, cycles: 4 },
                INSTRUCTION { name: "NOP", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 2 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::NOP, addrmode: cpu_addrmode::IMP, cycles: 4 },
                INSTRUCTION { name: "SBC", operate: cpu_ops::SBC, addrmode: cpu_addrmode::ABX, cycles: 4 },
                INSTRUCTION { name: "INC", operate: cpu_ops::INC, addrmode: cpu_addrmode::ABX, cycles: 7 },
                INSTRUCTION { name: "???", operate: cpu_ops::XXX, addrmode: cpu_addrmode::IMP, cycles: 7 },
            ],
        }
    }

    pub fn setup(&mut self) {
        self.bus.ram.iter_mut().for_each(|x| *x = 00);
    }

    pub fn read_ram(&mut self, addr: u16, _read_only: bool) -> u8 {
        self.bus.read(addr, _read_only)
    } 
    pub fn write_ram(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub fn get_flag(&self, flag: u8) -> u8 {
        if (self.status & flag) > 0 {
            1
        } else {
            0
        }
    }
    pub fn set_flag(&mut self, flag: u8, v: bool) {
        if v {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }
}

impl CPU {
    pub fn reset(&mut self) {
        self.addr_abs = 0xFFFC;
        let lo: u16 = self.read_ram(self.addr_abs, false).into();
        let hi: u16 = self.read_ram(self.addr_abs + 1, false).into();
        self.pc = (hi << 8) | lo;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = Flags::U as u8;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }

    fn irq(&mut self) {
        if self.get_flag(Flags::I as u8) == 0 {
            // Push the program counter to the stack. It's 16-bits dont
            // forget so that takes two pushes
            self.write_ram(0x0100 + self.stkp as u16, (self.pc >> 8 & 0x00FF) as u8);
            self.stkp -= 1;
            self.write_ram(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
            self.stkp -= 1;

            // Then Push the status register to the stack
            self.set_flag(Flags::B as u8, false);
            self.set_flag(Flags::U as u8, true);
            self.set_flag(Flags::I as u8, true);
            self.write_ram(0x0100 + self.stkp as u16, self.status);
            self.stkp -= 1;

            // Read new program counter location from fixed address
            self.addr_abs = 0xFFFE;
            let lo: u16 = self.read_ram(self.addr_abs, false).into();
            let hi: u16 = self.read_ram(self.addr_abs + 1, false).into();
            self.pc = (hi << 8) | lo;

            // IRQs take time
            self.cycles = 7;
        }
    }

    fn nmi(&mut self) {
        self.write_ram(0x0100 + self.stkp as u16, (self.pc >> 8 & 0x00FF) as u8);
        self.stkp -= 1;
        self.write_ram(0x0100 + self.stkp as u16, (self.pc & 0x00FF) as u8);
        self.stkp -= 1;

        self.set_flag(Flags::B as u8, false);
        self.set_flag(Flags::U as u8, true);
        self.set_flag(Flags::I as u8, true);
        self.write_ram(0x0100 + self.stkp as u16, self.status);
        self.stkp -= 1;

        self.addr_abs = 0xFFFA;
        let lo: u16 = self.read_ram(self.addr_abs, false).into();
        let hi: u16 = self.read_ram(self.addr_abs + 1, false).into();
        self.pc = (hi << 8) | lo;

        self.cycles = 8;
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read_ram(self.pc, false);
            self.set_flag(Flags::U as u8, true);
            self.pc += 1;
            self.cycles = self.lookup[self.opcode as usize].cycles;
            let additional_cycle1: u8 = (self.lookup[self.opcode as usize].addrmode)(self);
            let additional_cycle2: u8 = (self.lookup[self.opcode as usize].operate)(self);
            self.cycles += additional_cycle1 & additional_cycle2;
            self.set_flag(Flags::U as u8, true);
        }
        self.clock_count += 1;
        self.cycles -= 1;
    }

    pub fn fetch(&mut self) -> u8 {
        if self.lookup[self.opcode as usize].addrmode as usize != cpu_addrmode::IMP as usize {
            self.fetched = self.read_ram(self.addr_abs, false);
        }
        self.fetched
    }
}

impl CPU {
    pub fn complete(&mut self) -> bool {
        self.cycles == 0
    }
    pub fn dissasemble(&mut self, nStart: u16, nStop: u16) -> Vec<(u16, String)> {
        let mut addr: u32 = nStart as u32;
        let mut mapLines = Vec::new();


        while addr <= nStop as u32 {
            let line_addr = addr as u16;
            let mut sInst = format!("${}: ", format!("{:04X}", addr));
            
            let op_for_disassemble: u8 = self.read_ram(line_addr as u16, true);
            addr += 1;
            sInst = format!("{}{} ", sInst, self.lookup[op_for_disassemble as usize].name);
            
            if self.lookup[op_for_disassemble as usize].addrmode as usize == cpu_addrmode::IMP as usize {
                sInst += " {IMP}";
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::IMM as usize
            {
                let value = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}#${} {{IMP}}", sInst, format!("{:02X}", value));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ZP0 as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${} {{ZP0}}", sInst, format!("{:02X}", lo));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ZPX as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${}, X {{ZPX}}", sInst, format!("{:02X}", lo));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ZPY as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${}, Y {{ZPY}}", sInst, format!("{:02X}", lo));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::IZX as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}(${}, X) {{IZX}}", sInst, format!("{:02X}", lo));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::IZY as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}(${}), Y {{IZY}}", sInst, format!("{:02X}", lo));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ABS as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                let hi = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${} {{ABS}}", sInst, format!("{:04X}", ((hi as u16) << 8) | lo as u16));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ABX as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                let hi = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${}, X {{ABX}}", sInst, format!("{:04X}", ((hi as u16) << 8) | lo as u16));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::ABY as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                let hi = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}${}, Y {{ABY}}", sInst, format!("{:04X}", ((hi as u16) << 8) | lo as u16));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::IND as usize
            {
                let lo = self.read_ram(addr as u16, true);
                addr += 1;
                let hi = self.read_ram(addr as u16, true);
                addr += 1;
                sInst = format!("{}(${}) {{IND}}", sInst, format!("{:04X}", ((hi as u16) << 8) | lo as u16));
            } else if self.lookup[op_for_disassemble as usize].addrmode as usize
                == cpu_addrmode::REL as usize
            {
                let value = self.read_ram(addr as u16, true);
                println!("addr {:04X} val {:04X}", addr, value);
                addr += 1;
                println!("addr {:04X} val {:04X}", addr, value);
                sInst = format!(
                    "{}${} [${}] {{REL}}",
                    sInst,
                    format!("{:02X}", value),
                    format!("{:04X}", (addr.wrapping_add((value as i8) as u32)))
                );
            }
// if addr as u16 == 0x8000 {println!("tuple {:?}", (line_addr, sInst.));};
            mapLines.push((line_addr, sInst));
        }

        mapLines
    }
}


