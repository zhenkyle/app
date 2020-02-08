//#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
extern crate panic_itm; // logs messages over ITM; requires ITM support
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

#[allow(unused_imports)]
use cortex_m::{iprint, iprintln};
use cortex_m_rt::entry;
use f3::hal::{
    prelude::*,
    serial,
    stm32,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];
    iprintln!(stim, "Hello, world!");

    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    serial::Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    unsafe {
        let usart1 = &*stm32::USART1::ptr();
        for byte in b"The quick brown fox jumps over the lazy dog.".iter() {
             usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
         }
    }



    loop {}
}
