//#![deny(unsafe_code)] // usart1.tdr.write is unsafe
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
use heapless::{Vec, consts};

#[entry]
fn main() -> ! {
    let (usart1, _itm, _mono_timer) = app::init();
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            // Wait until there's data available
            while usart1.isr.read().rxne().bit_is_clear() {}

            // Retrieve the data
            let byte = usart1.rdr.read().rdr().bits() as u8;

            if byte != b'\r' {
                if buffer.push(byte).is_err() {
                    for byte in b"error: buffer full\n\r" {
                        while usart1.isr.read().txe().bit_is_clear() {}
                        unsafe {
                            usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                        }
                    }
                    break;
                };
                continue;
            }

            for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                while usart1.isr.read().txe().bit_is_clear() {}
                unsafe {
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
                }
            }
            break;
        }
    }
}

