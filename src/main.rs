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
    const XY_GAIN: f32 = 1100.; // LSB, unit G
    const Z_GAIN: f32 = 980.;    // LSB, unit G
    
    let (mut _leds, mut lsm303dlhc, mut delay, mut itm) = app::init();
    
    loop {
        let lsm303dlhc::I16x3 { x, y, z } = lsm303dlhc.mag().unwrap();

        let x = f32::from(x) / XY_GAIN;
        let y = f32::from(y) / XY_GAIN;
        let z = f32::from(z) / Z_GAIN;

        let mag = (x * x + y * y + z * z).sqrt();

        iprintln!(&mut itm.stim[0], "{} mG", mag * 1_000.);
        delay.delay_ms(500_u16);
    }
}

