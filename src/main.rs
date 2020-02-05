#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use f3::{
    hal::{prelude::*, stm32},
    led::Leds,
};

#[entry]
fn main() -> ! {
    //    let p = cortex_m::Peripherals::take().unwrap();
    let p = stm32::Peripherals::take().unwrap();

    // RCC: reset and clock control register
    let mut rcc = p.RCC.constrain();
    // ahb: AHB stands for Advanced High-performance Bus and APB sands for Advanced Peripheral Bus.
    // http://www.differencebetween.net/technology/difference-between-ahb-and-apb/
    // p.GPIOE.split split a GPIO peripheral in independent pins and registers
    /*
pub struct Parts {
    pub afrh: AFRH,
    pub afrl: AFRL,
    pub moder: MODER,
    pub otyper: OTYPER,
    pub pupdr: PUPDR,
    pub pe0: PE0<Input<Floating>>,
    pub pe1: PE1<Input<Floating>>,
    pub pe2: PE2<Input<Floating>>,
    pub pe3: PE3<Input<Floating>>,
    pub pe4: PE4<Input<Floating>>,
    pub pe5: PE5<Input<Floating>>,
    pub pe6: PE6<Input<Floating>>,
    pub pe7: PE7<Input<Floating>>,
    pub pe8: PE8<Input<Floating>>,
    pub pe9: PE9<Input<Floating>>,
    pub pe10: PE10<Input<Floating>>,
    pub pe11: PE11<Input<Floating>>,
    pub pe12: PE12<Input<Floating>>,
    pub pe13: PE13<Input<Floating>>,
    pub pe14: PE14<Input<Floating>>,
    pub pe15: PE15<Input<Floating>>,
}
*/
    let gpioe = p.GPIOE.split(&mut rcc.ahb);
    let mut leds = Leds::new(gpioe);

    for led in leds.iter_mut() {
        led.on();
    }


    loop {
        // your code goes here
    }
}

