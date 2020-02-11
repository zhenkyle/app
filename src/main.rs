#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

use f3::hal::prelude::*; // for DelayMs
use f3::lsm303dlhc;
use m::Float;

#[entry]
fn main() -> ! {
    const SENSITIVITY: f32 = 12. / (1 << 14) as f32; // 16 bit 1 bit sign, 15 significent bit for 12g
    const THRESHOLD: f32 = 0.5;
    
    let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = app::init();

    // extend sensing rang to `[-12g, +12g]`
    lsm303dlhc.set_accel_sensitivity(lsm303dlhc::Sensitivity::G12).unwrap();

    let measurement_time = mono_timer.frequency().0; // 8,000,000 ticks, that is 1 second
    // instant have three state:
    // (1) None: below threshold not measuring
    // (2) Some if < 1 second: above threshold , start measuring
    // (3) Somer >=1 sencod: outputing result, end measuring
    let mut instant = None;
    let mut max_g = 0.;
    loop {
        let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;

        match instant {
            None => {
                if g_x > THRESHOLD {
                    iprintln!(&mut itm.stim[0], "START!");
                    
                    max_g = g_x;
                    instant = Some(mono_timer.now());
                }
            }
            // destruct Option instant to Mono_timer instant, ref means: instant = & mono_timer.now() // right ?
            Some(ref instant)  if instant.elapsed() < measurement_time => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // Report max value
                iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

                // Measurement done
                instant = None;

                // Reset
                max_g = 0.;
            }
        }

        delay.delay_ms(50_u8);
    }
}

