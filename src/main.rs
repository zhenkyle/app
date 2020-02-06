#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
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
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut leds = led::Leds::new(gpioe);
    
    loop {
        for curr in 0 .. leds.len() {
            let next = (curr + 1) % leds.len();

            leds[next].on();
            delay();
            leds[curr].off();
            delay();
        }
    }
}

fn delay() {
    for _ in 1..10_000 {
        asm::nop();
    }
}
