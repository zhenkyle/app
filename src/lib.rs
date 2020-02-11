#![no_std]

extern crate panic_itm; // panic handler

use f3::hal;
use f3::hal::prelude::*;
use f3::hal::stm32;
use f3::led;
use f3::Lsm303dlhc;


pub fn init()->(led::Leds, Lsm303dlhc, hal::delay::Delay, cortex_m::peripheral::ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr); // mind the stm32f3xx-hal default clock configuration BUG

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let leds = led::Leds::new(gpioe);
    
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = hal::i2c::I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    let delay = hal::delay::Delay::new(cp.SYST, clocks);

    (leds, lsm303dlhc, delay, cp.ITM)
}
