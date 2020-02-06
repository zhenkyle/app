#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm};
use cortex_m_rt::entry;
use f3:: {
    hal:: {
        stm32,
    }
};

#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().unwrap();
    let rcc = &dp.RCC;
    rcc.ahbenr.modify(|_,w| w.iopeen().set_bit());

    let gpioe = &dp.GPIOE;
    gpioe.moder.modify(|_, w| {
        w.moder9().output();
        w.moder11().output()
    });
    
    gpioe.bsrr.write(|w| w.bs9().set_bit());
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    
    asm::bkpt();

    loop {}
}

