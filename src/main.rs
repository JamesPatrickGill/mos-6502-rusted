mod drawing;
mod globals;

mod bus;
mod cpu;
mod cpu_addrmode;
mod cpu_ops;
mod emulator;

use pixels::Error;

fn main() -> Result<(), Error> {
    // let times_prog = "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA";
    let sqrt_prog = "A2 00 8E 00 00 A2 40 8E 01 00 A9 00 8D 10 00 8D 11 00 A2 08 38 AD 00 00 E9 40 A8 AD 11 00 ED 10 00 90 06 8C 00 00 8D 11 00 2E 10 00 0E 01 00 2E 00 00 2E 11 00 0E 01 00 2E 00 00 2E 11 00 CA D0 D3";
    emulator::mos_6502_emulator(sqrt_prog)
}
