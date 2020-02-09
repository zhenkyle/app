#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

#[allow(unused_imports)]
use app::init;

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Address of magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, _delay, mut itm) = app::init();

    let byte = {
        0
    };

    iprintln!(&mut itm.stim[0], "0x:{:02X} - 0b{:08b}", IRA_REG_M, byte);
    loop {}
}

