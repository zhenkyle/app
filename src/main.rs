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

    stm32::Peripherals::take().unwrap();

    unsafe {
        let rcc = &*stm32::RCC::ptr();
        rcc.ahbenr.modify(|_,w| w.iopeen().set_bit());

        let gpioe = &*stm32::GPIOE::ptr();
        // configure the pins as outputs
        gpioe.moder.modify(|_, w| {
            w.moder9().output();
            w.moder11().output()
        });

        // Way 1
        const GPIOE_BSRR: u32 = 0x48001018;
        asm::bkpt();
        *(GPIOE_BSRR as *mut u32) = 1 << 9;
        *(GPIOE_BSRR as *mut u32) = 1 << 11;
        // Note: to turn multiply leds, use:
        // core::ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        
        // Way 2
        /*
        gpioe.bsrr.write(|w| w.bs9().set_bit());
        */

        // Way 3
        /*
        gpioe.odr.write(|w| {
            w.odr9().set_bit()
        });
        */
    }

    loop {}
}

