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
    led::Led,
};

#[entry]
fn main() -> ! {

    let (mut led, mut delay) = init();
    loop {
        led.on();
        delay.delay_ms(1_000_u16);
        led.off();
        delay.delay_ms(1_000_u16);
    }
}

fn init()-> (Led,Delay) {
    // cp: cortex_m Peripheral
    let cp = cortex_m::Peripherals::take().unwrap();
    // dp: stm32 Peripheral
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    // RCC: reset and clock control register
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr); //alternativly
    /* clocks's structure
pub struct Clocks {
    hclk: Hertz, // AHB clock
    pclk1: Hertz, // APB1 clock
    pclk2: Hertz, // APB2 clock
    ppre1: u8, // APB1 high-speed prescaler 
    ppre2: u8, // APB2 high-speed prescaler
    sysclk: Hertz, // SysTick
    usbclk_valid: bool, // whether the USBCLK clock frequency is valid for the USB peripheral
}
     */
    let delay = Delay::new(cp.SYST, clocks);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let led: Led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
        .into();

    (led, delay)

}
