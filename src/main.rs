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
    let (mut _leds, mut lsm303dlhc, mut delay, mut itm) = app::init();
    
    loop {
        let lsm303dlhc::I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        iprintln!(&mut itm.stim[0], "{}\t{}\t{}", x, y, z);
        
        delay.delay_ms(100_u8);
    }
}

