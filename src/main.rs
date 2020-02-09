#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
extern crate panic_itm; // logs messages over ITM; requires ITM support
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

#[allow(unused_imports)]
use cortex_m::{iprintln};
use cortex_m_rt::entry;
use app::{uprintln,uprint,SerialPort};
use core::fmt::Write; // need this to enable $serial.write_fmt

#[entry]
fn main() -> ! {
    let (usart1, mut itm, mono_timer) = app::init();
    let stim = &mut itm.stim[0];

    let mut serial = SerialPort{usart1};

    let instant = mono_timer.now();
    uprintln!(serial, "The answer is {}", 40 + 2);
    let elapsed = instant.elapsed();

    iprintln!(stim,
              "for loop took {} ticks ({} us)",
              elapsed,
              elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
    );

    loop {}
}

