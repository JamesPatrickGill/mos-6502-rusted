#![allow(dead_code)]
#![allow(non_snake_case)]

pub mod chars {
    pub const A: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1111_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const B: [u8; 8] = [
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1111_0000,
        0b0000_0000,
    ];
    pub const C: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const D: [u8; 8] = [
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1111_0000,
        0b0000_0000,
    ];
    pub const E: [u8; 8] = [
        0b1111_1000,
        0b1000_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const F: [u8; 8] = [
        0b1111_1000,
        0b1000_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b0000_0000,
    ];
    pub const G: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_0000,
        0b1000_0000,
        0b1001_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const H: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1111_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const I: [u8; 8] = [
        0b1111_1000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const J: [u8; 8] = [
        0b0011_1000,
        0b0001_0000,
        0b0001_0000,
        0b0001_0000,
        0b0001_0000,
        0b1001_0000,
        0b0110_0000,
        0b0000_0000,
    ];
    pub const K: [u8; 8] = [
        0b1000_1000,
        0b1001_0000,
        0b1010_0000,
        0b1100_0000,
        0b1010_0000,
        0b1001_0000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const L: [u8; 8] = [
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const M: [u8; 8] = [
        0b1000_1000,
        0b1101_1000,
        0b1010_1000,
        0b1010_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const N: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b1100_1000,
        0b1010_1000,
        0b1001_1000,
        0b1000_1000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const O: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const P: [u8; 8] = [
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1111_0000,
        0b1000_0000,
        0b1000_0000,
        0b1000_0000,
        0b0000_0000,
    ];
    pub const Q: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1010_1000,
        0b1001_0000,
        0b0110_1000,
        0b0000_0000,
    ];
    pub const R: [u8; 8] = [
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b1111_0000,
        0b1010_0000,
        0b1001_0000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const S: [u8; 8] = [
        0b0111_1000,
        0b1000_0000,
        0b1000_0000,
        0b0111_0000,
        0b0000_1000,
        0b0000_1000,
        0b1111_0000,
        0b0000_0000,
    ];
    pub const T: [u8; 8] = [
        0b1111_1000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0000_0000,
    ];
    pub const U: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const V: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b0101_0000,
        0b0101_0000,
        0b0010_0000,
        0b0000_0000,
    ];
    pub const W: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b1000_1000,
        0b1010_1000,
        0b1010_1000,
        0b1010_1000,
        0b0101_0000,
        0b0000_0000,
    ];
    pub const X: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b0101_0000,
        0b0010_0000,
        0b0101_0000,
        0b1000_1000,
        0b1000_1000,
        0b0000_0000,
    ];
    pub const Y: [u8; 8] = [
        0b1000_1000,
        0b1000_1000,
        0b0101_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0000_0000,
    ];
    pub const Z: [u8; 8] = [
        0b1111_1000,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1000_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const SPACE: [u8; 8] = [
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ];
    pub const ZERO: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1001_1000,
        0b1010_1000,
        0b1100_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const ONE: [u8; 8] = [
        0b0010_0000,
        0b1110_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const TWO: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b1111_1000,
        0b0000_0000,
    ];
    pub const THREE: [u8; 8] = [
        0b1111_1000,
        0b0001_0000,
        0b0010_0000,
        0b0001_0000,
        0b0000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const FOUR: [u8; 8] = [
        0b0001_0000,
        0b0011_0000,
        0b0101_0000,
        0b1001_0000,
        0b1111_1000,
        0b0001_0000,
        0b0001_0000,
        0b0000_0000,
    ];
    pub const FIVE: [u8; 8] = [
        0b1111_1000,
        0b1000_0000,
        0b1111_0000,
        0b0000_1000,
        0b0000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const SIX: [u8; 8] = [
        0b0011_0000,
        0b0100_0000,
        0b1000_0000,
        0b1111_0000,
        0b1000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const SEVEN: [u8; 8] = [
        0b1111_1000,
        0b0000_1000,
        0b0001_0000,
        0b0010_0000,
        0b0100_0000,
        0b0100_0000,
        0b0100_0000,
        0b0000_0000,
    ];
    pub const EIGHT: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b0111_0000,
        0b0000_0000,
    ];
    pub const NINE: [u8; 8] = [
        0b0111_0000,
        0b1000_1000,
        0b1000_1000,
        0b0111_1000,
        0b0000_1000,
        0b0001_0000,
        0b0110_0000,
        0b0000_0000,
    ];
    pub const DOLLAR: [u8; 8] = [
        0b0010_0000,
        0b0111_1000,
        0b1010_0000,
        0b0111_0000,
        0b0010_1000,
        0b1111_0000,
        0b0010_0000,
        0b0000_0000,
    ];
    pub const COLON: [u8; 8] = [
        0b0000_0000,
        0b0100_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0100_0000,
        0b0000_0000,
        0b0000_0000,
    ];
    pub const LBRACE: [u8; 8] = [
        0b0001_1000,
        0b0010_0000,
        0b0010_0000,
        0b0100_0000,
        0b0010_0000,
        0b0010_0000,
        0b0001_1000,
        0b0000_0000,
    ];
    pub const RBRACE: [u8; 8] = [
        0b1100_0000,
        0b0010_0000,
        0b0010_0000,
        0b0001_0000,
        0b0010_0000,
        0b0010_0000,
        0b1100_0000,
        0b0000_0000,
    ];
    pub const LSQR: [u8; 8] = [
        0b0011_1000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0011_1000,
        0b0000_0000,
    ];
    pub const RSQR: [u8; 8] = [
        0b1110_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b0010_0000,
        0b1110_0000,
        0b0000_0000,
    ];
    pub const HASH: [u8; 8] = [
        0b0101_0000,
        0b0101_0000,
        0b1111_1000,
        0b0101_0000,
        0b1111_1000,
        0b0101_0000,
        0b0101_0000,
        0b0000_0000,
    ];
    pub const DASH: [u8; 8] = [
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b1111_1000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
        0b0000_0000,
    ];

    pub fn get_letter_from_char(letter_char: char) -> [u8; 8] {
        match letter_char {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            'E' => E,
            'F' => F,
            'G' => G,
            'H' => H,
            'I' => I,
            'J' => J,
            'K' => K,
            'L' => L,
            'M' => M,
            'N' => N,
            'O' => O,
            'P' => P,
            'Q' => Q,
            'R' => R,
            'S' => S,
            'T' => T,
            'U' => U,
            'V' => V,
            'W' => W,
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            '0' => ZERO,
            '1' => ONE,
            '2' => TWO,
            '3' => THREE,
            '4' => FOUR,
            '5' => FIVE,
            '6' => SIX,
            '7' => SEVEN,
            '8' => EIGHT,
            '9' => NINE,
            '$' => DOLLAR,
            ':' => COLON,
            '{' => LBRACE,
            '}' => RBRACE,
            '[' => LSQR,
            ']' => RSQR,
            '#' => HASH,
            '-' => DASH,
            ' ' => SPACE,

            _ => Z,
        }
    }
}

pub mod basics {
    use bit_vec::BitVec;
    use hex;

    pub fn draw_letter(
        frame: &mut [u8],
        letter_char: char,
        x_offset: usize,
        y_offset: usize,
        colour: &str,
    ) {
        let letter_bit_array: BitVec =
            BitVec::from_bytes(&super::chars::get_letter_from_char(letter_char));
        let rgb_bytes = hex::decode(colour).expect("Decoding failed");
        for (index, bit) in letter_bit_array.iter().enumerate() {
            if bit {
                let x = index % 8;
                let y = index / 8;

                let place = (x + x_offset % crate::globals::WIDTH as usize)
                    + (y + y_offset % crate::globals::HEIGHT as usize)
                        * crate::globals::WIDTH as usize;

                frame[place * 4] = rgb_bytes[0];
                frame[place * 4 + 1] = rgb_bytes[1];
                frame[place * 4 + 2] = rgb_bytes[2];
                frame[place * 4 + 3] = rgb_bytes[3];
            }
        }
    }

    pub fn draw_string(
        frame: &mut [u8],
        string: &str,
        x_offset: usize,
        y_offset: usize,
        colour: &str,
    ) {
        for (index, character) in string.chars().enumerate() {
            let x = x_offset + index * 6;
            let y = y_offset;
            draw_letter(frame, character, x, y, colour)
        }
    }

    pub fn draw_line(
        frame: &mut [u8],
        p1: (isize, isize),
        p2: (isize, isize),
        x_offset: isize,
        y_offset: isize,
        colour: &str,
    ) {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        let rgb_bytes = hex::decode(colour).expect("Decoding failed");

        if dx == 0 {
            let mut start = p1.1;
            let mut end = p2.1;

            if p2.1 < p1.1 {
                start = p2.1;
                end = p1.1
            };

            for pos_y in start..end {
                let place = (p1.0 + x_offset % crate::globals::WIDTH as isize)
                    + (pos_y + y_offset % crate::globals::HEIGHT as isize)
                        * crate::globals::WIDTH as isize;

                frame[(place * 4) as usize] = rgb_bytes[0];
                frame[(place * 4 + 1) as usize] = rgb_bytes[1];
                frame[(place * 4 + 2) as usize] = rgb_bytes[2];
                frame[(place * 4 + 3) as usize] = rgb_bytes[3];
            }
        }

        for pos_x in p1.0..p2.0 {
            let pos_y = p2.1 + dy * (pos_x - p1.0) / dx;

            let place = (pos_x + x_offset % crate::globals::WIDTH as isize)
                + (pos_y + y_offset % crate::globals::HEIGHT as isize)
                    * crate::globals::WIDTH as isize;

            frame[(place * 4) as usize] = rgb_bytes[0];
            frame[(place * 4 + 1) as usize] = rgb_bytes[1];
            frame[(place * 4 + 2) as usize] = rgb_bytes[2];
            frame[(place * 4 + 3) as usize] = rgb_bytes[3];
        }
    }
}

pub mod debugger {
    pub fn draw_ram(
        frame: &mut [u8],
        cpu: &mut crate::cpu::CPU,
        x: u32,
        y: u32,
        nAddr: &mut u16,
        nRows: u16,
        nColumns: u16,
    ) {
        let nRamX = x;
        let mut nRamY = y;
        for _rows in 0..nRows {
            let mut sOffset = format!("${}:", format!("{:04X}", nAddr));
            for _col in 0..nColumns {
                sOffset = format!(
                    "{} {}",
                    sOffset,
                    format!("{:02X}", cpu.read_ram(*nAddr, true))
                );
                *nAddr += 1;
            }
            super::basics::draw_string(frame, &sOffset, nRamX as usize, nRamY as usize, "FECEABFF");
            nRamY += 10;
        }
    }
    pub fn draw_cpu(frame: &mut [u8], cpu: &mut crate::cpu::CPU, x: usize, y: usize) {
        super::basics::draw_string(frame, &format!("     A: ${:02X}", cpu.a), x, y, "FECEABFF");
        super::basics::draw_string(
            frame,
            &format!("     X: ${:02X}", cpu.x),
            x,
            y + 10,
            "FECEABFF",
        );
        super::basics::draw_string(
            frame,
            &format!("     Y: ${:02X}", cpu.y),
            x,
            y + 20,
            "FECEABFF",
        );
        super::basics::draw_string(
            frame,
            &format!("    PC: ${:04X}", cpu.pc),
            x,
            y + 30,
            "FECEABFF",
        );
        super::basics::draw_string(
            frame,
            &format!("  STKP: ${:04X}", cpu.stkp),
            x,
            y + 40,
            "FECEABFF",
        );
        super::basics::draw_string(frame, "STATUS:", x, y + 60, "FECEABFF");
        super::basics::draw_string(
            frame,
            "N",
            x + 48,
            y + 60,
            if cpu.status & crate::cpu::Flags::N as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "V",
            x + 60,
            y + 60,
            if cpu.status & crate::cpu::Flags::V as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "U",
            x + 72,
            y + 60,
            if cpu.status & crate::cpu::Flags::U as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "B",
            x + 84,
            y + 60,
            if cpu.status & crate::cpu::Flags::B as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "D",
            x + 96,
            y + 60,
            if cpu.status & crate::cpu::Flags::D as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "I",
            x + 108,
            y + 60,
            if cpu.status & crate::cpu::Flags::I as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "Z",
            x + 120,
            y + 60,
            if cpu.status & crate::cpu::Flags::Z as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            "C",
            x + 132,
            y + 60,
            if cpu.status & crate::cpu::Flags::C as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(frame, "IN BIN:", x, y + 70, "FECEABFF");
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::N as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 48,
            y + 70,
            if cpu.status & crate::cpu::Flags::N as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::V as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 60,
            y + 70,
            if cpu.status & crate::cpu::Flags::V as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::U as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 72,
            y + 70,
            if cpu.status & crate::cpu::Flags::U as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::B as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 84,
            y + 70,
            if cpu.status & crate::cpu::Flags::B as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::D as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 96,
            y + 70,
            if cpu.status & crate::cpu::Flags::D as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::I as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 108,
            y + 70,
            if cpu.status & crate::cpu::Flags::I as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::Z as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 120,
            y + 70,
            if cpu.status & crate::cpu::Flags::Z as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
        super::basics::draw_string(
            frame,
            if cpu.status & crate::cpu::Flags::C as u8 != 0 {
                "1"
            } else {
                "0"
            },
            x + 132,
            y + 70,
            if cpu.status & crate::cpu::Flags::C as u8 != 0 {
                "99B898FF"
            } else {
                "E84A5FFF"
            },
        );
    }

    pub fn draw_code(
        frame: &mut [u8],
        codeMap: &mut Vec<(u16, String)>,
        cpu: &mut crate::cpu::CPU,
        x: usize,
        y: usize,
        nLines: u8,
    ) {
        let mut iter_a_end = 0;
        for (index, item) in codeMap.iter().enumerate() {
            if item.0 == cpu.pc {
                iter_a_end = index;
                break;
            }
            if index == codeMap.len() - 1 {
                iter_a_end = index;
                break;
            }
        }
        let mut nLineY = ((nLines >> 1) * 10) as usize + y;
        if iter_a_end != codeMap.len() - 1 {
            super::basics::draw_string(
                frame,
                &codeMap[iter_a_end].1,
                x,
                nLineY as usize,
                "99B898FF",
            );
            while nLineY < (nLines * 10) as usize + y {
                nLineY += 10;
                iter_a_end += 1;
                if iter_a_end != codeMap.len() - 1 {
                    super::basics::draw_string(
                        frame,
                        &codeMap[iter_a_end].1,
                        x,
                        nLineY as usize,
                        "FECEABFF",
                    );
                }
            }
        }

        for (index, item) in codeMap.iter().enumerate() {
            if item.0 == cpu.pc {
                iter_a_end = index;
                break;
            }
            if index == codeMap.len() - 1 {
                iter_a_end = index;
                break;
            }
        }
        nLineY = ((nLines >> 1) * 10) as usize + y;
        if iter_a_end != codeMap.len() - 1 {
            while nLineY > y {
                nLineY -= 10;
                iter_a_end = iter_a_end.wrapping_sub(1);
                if iter_a_end != codeMap.len() - 1 {
                    super::basics::draw_string(
                        frame,
                        &codeMap[iter_a_end].1,
                        x,
                        nLineY as usize,
                        "FECEABFF",
                    );
                }
            }
        }
    }
    pub fn draw_debug(
        frame: &mut [u8],
        cpu: &mut crate::cpu::CPU,
        code_map: &mut Vec<(u16, String)>,
    ) {
        frame.iter_mut().for_each(|x| *x = 00);
        super::basics::draw_string(frame, "6502-EMULATOR", 100, 2, "FF847CFF");
        super::basics::draw_line(frame, (2, 5), (98, 5), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (179, 5), (532, 5), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (2, 5), (2, 368), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (2, 368), (533, 368), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (532, 5), (532, 368), 0, 0, "FECEABFF");

        super::basics::draw_string(frame, "RAM - PAGES 0 AND 80", 26, 18, "FF847CFF");
        super::basics::draw_line(frame, (8, 21), (24, 21), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (147, 21), (343, 21), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (8, 21), (8, 362), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (8, 362), (343, 362), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (342, 362), (342, 21), 0, 0, "FECEABFF");
        draw_ram(frame, cpu, 14, 30, &mut 0x0000, 16, 16);
        draw_ram(frame, cpu, 14, 200, &mut 0x8000, 16, 16);

        super::basics::draw_string(frame, "CPU STATE", 370, 18, "FF847CFF");
        super::basics::draw_line(frame, (350, 21), (368, 21), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (425, 21), (496, 21), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (350, 21), (350, 110), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (350, 110), (498, 110), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (497, 21), (497, 110), 0, 0, "FECEABFF");
        draw_cpu(frame, cpu, 356, 30);

        super::basics::draw_string(frame, "DISASSEMBLY", 370, 117, "FF847CFF");
        super::basics::draw_line(frame, (350, 120), (368, 120), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (437, 120), (526, 120), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (350, 120), (350, 362), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (350, 362), (527, 362), 0, 0, "FECEABFF");
        super::basics::draw_line(frame, (526, 120), (526, 362), 0, 0, "FECEABFF");
        draw_code(frame, code_map, cpu, 356, 130, 22);
    }
}
