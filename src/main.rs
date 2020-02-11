#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

use f3::hal::prelude::*; // for DelayMs
use f3::lsm303dlhc;


#[entry]
fn main() -> ! {
    let (mut lsm303dlhc, mut delay, _mono_timer, mut itm) = app::init();

    // extend sensing range to `[-12g, +12g]`
    lsm303dlhc.set_accel_sensitivity(lsm303dlhc::Sensitivity::G12).unwrap();
    loop {
        const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
        
        let lsm303dlhc::I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

        let x = f32::from(x) * SENSITIVITY;
        let y = f32::from(y) * SENSITIVITY;
        let z = f32::from(z) * SENSITIVITY;
        
        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
        
        delay.delay_ms(1_000_u16);
    }
}

