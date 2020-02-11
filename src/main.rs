#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

#[allow(unused_imports)]
use app::init;
//use f3::hal::hal::blocking::delay::DelayMs;
use f3::hal::prelude::*; // for DelayMs
use f3::lsm303dlhc;
use f3::led;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut itm) = app::init();
    loop {
        // Note: I16x3 is a struct, for Destructing Structs
        // See https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
        let lsm303dlhc::I16x3 { x, y, ..} = lsm303dlhc.mag().unwrap();

        let dir = match (x >0, y>0) {
            (true, true) => led::Direction::Southeast,
            (false, true) => led::Direction::Northeast,
            (false, false) => led::Direction::Northwest,
            (true,false) => led::Direction::Southwest,
        };

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(1_000_u16);
    }
}

