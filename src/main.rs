#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use f3:: {
    hal:: {
        prelude::*,
        stm32,
    },
    led,
};

#[entry]
fn main() -> ! {

    let dp = stm32::Peripherals::take().unwrap();

    dp.RCC.apb1enr.modify(|_,w| w.tim6en().set_bit());
    let tim6 = &dp.TIM6;
    // OPM select one plus mode
    // CEN keep the counter disabled for now
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    // Configure the prescaler to have the counter operate at 1 KHz
    tim6.psc.write(|w| w.psc().bits(7_999));
    
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut leds = led::Leds::new(gpioe);

    loop {
        for curr in 0 .. leds.len() {
            let next = (curr + 1) % leds.len();

            leds[next].on();
            delay(tim6, 50);
            leds[curr].off();
            delay(tim6, 50);
        }
    }
}

fn delay(tim6: &stm32::TIM6, ms: u16) {
    unsafe {
        // Set the timer to go off in ms miniseconds
        tim6.arr.write(|w| w.arr().bits(ms));
    }
    // CEN: Enable the counter
    tim6.cr1.modify(|_, w| w.cen().set_bit());
    // Wait until the alarm goes off
    while !tim6.sr.read().uif().bit_is_set() {}
    // Clear the update envent flag
    tim6.sr.modify(|_,w| w.uif().clear_bit());

}
