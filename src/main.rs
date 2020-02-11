#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

use app::init;
//use f3::hal::hal::blocking::delay::DelayMs;
use f3::hal::prelude::*; // for DelayMs
use f3::lsm303dlhc;
use f3::led;

use core::f32::consts::PI;

use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut _itm) = app::init();
    loop {
        // Note: I16x3 is a struct, for Destructing Structs
        // See https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
        let lsm303dlhc::I16x3 { x, y, ..} = lsm303dlhc.mag().unwrap();

        let _theta = (y as f32).atan2(x as f32); // in radians

        // FIXME
        let dir = led::Direction::Southeast;
        
        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(1_000_u16);
    }
}

