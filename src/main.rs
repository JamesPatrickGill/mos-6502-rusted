mod drawing;
mod globals;

mod bus;
mod cpu;
mod cpu_addrmode;
mod cpu_ops;
mod emulator;

use pixels::Error;

fn main() -> Result<(), Error> {
    // let times_prog =
    //     "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA";
    let sqrt_prog = "A2 90 8E 00 00 A2 01 8E 01 00 A9 00 8D 02 00 8D 03 00
8D 06 00 A2 08 0E 06 00
0E 00 00 2E 01 00 2E 02
00 2E 03 00 0E 00 00 2E
01 00 2E 02 00 2E 03 00
AD 06 00 8D 04 00 A9 00
8D 05 00 38 2E 04 00 2E
05 00 AD 03 00 CD 05 00
90 1F D0 08 AD 03 00 CD
04 00 90 15 AD 02 00 ED
04 00 8D 02 00 AD 03 00
ED 05 00 8D 03 00 EE 06
00 CA D0 A9 60
";
    emulator::mos_6502_emulator(sqrt_prog)
}
