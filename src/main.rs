#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use f3::{
    hal::{delay::Delay, prelude::*, stm32},
    led::Leds,
};

#[entry]
fn main() -> ! {

    let (mut leds, mut delay) = init();
    let n = leds.len();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();

            delay.delay_ms(100_u8);
        }
    }
}

fn init()-> (Leds,Delay) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(cp.SYST, clocks);

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let leds: Leds = Leds::new(gpioe);
    (leds, delay)

}
