#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
extern crate panic_itm; // logs messages over ITM; requires ITM support
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
#[allow(unused_imports)]
use cortex_m::{iprintln, asm};
#[allow(unused_imports)]
use app::{uprintln,uprint,SerialPort};
#[allow(unused_imports)]
use core::fmt::Write; // need this to enable $serial.write_fmt

#[entry]
fn main() -> ! {
    let (usart1, _itm, _mono_timer) = app::init();

    loop {
        // Wait until there's data available
        while usart1.isr.read().rxne().bit_is_clear() {}

        // Retrieve the data
        let _byte = usart1.rdr.read().rdr().bits() as u8;

        asm::bkpt();
    }
}

