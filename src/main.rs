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

        let theta = (y as f32).atan2(x as f32); // in radians
        let mut dir = led::Direction::Southeast;
        if theta >= - PI/8.0 && theta < PI/8.0 {
            dir = led::Direction::South;
        } else if theta >= PI/8.0 && theta < PI * 3.0 / 8.0 {
            dir = led::Direction::Southeast;
        } else if theta >= 3.0 * PI/ 8.0 && theta < PI * 5.0 / 8.0 {
            dir = led::Direction::East;
        } else if theta >= PI * 5.0 / 8.0 && theta < PI * 7.0 / 8.0 {
            dir = led::Direction::Northeast;
        } else if theta >= PI * 7.0 / 8.0 && theta <= PI ||
        theta > -PI && theta < -PI * 7.0 / 8.0 {
            dir = led::Direction::North;
        } else if theta >= -PI * 7.0 / 8.0 && theta < - PI * 5.0 / 8.0 {
            dir = led::Direction::Northwest;
        } else if theta >= -PI * 5.0 / 8.0 && theta < - PI * 3.0 / 8.0 {
            dir = led::Direction::West;
        } else {
            dir = led::Direction::Southwest;
        }
        
        
        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(1_000_u16);
    }
}

