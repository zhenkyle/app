#![no_std]

use f3::hal::{
    prelude::*,
    serial,
    stm32,
    time,
};


#[macro_export]
macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        // ok() turn Result to Option, the point is USE Result,
        // not interested in Option, only interested in side effect
        // of write_fmt.
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

#[macro_export]
macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n\r"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n\r"), $($arg)*)
    }
}

pub struct SerialPort {
    pub usart1: &'static mut stm32::usart1::RegisterBlock,
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
    for byte in s.as_bytes().iter() {
        while self.usart1.isr.read().txe().bit_is_clear() {}
        unsafe {
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(*byte)));
         };
    }
        Ok(())
    }
}

pub fn init()->(&'static mut stm32::usart1::RegisterBlock ,
            cortex_m::peripheral::ITM,
            time::MonoTimer) {
    let cp = cortex_m::Peripherals::take().unwrap();

    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    serial::Serial::usart1(dp.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    let mono_timer = time::MonoTimer::new(cp.DWT, clocks);

    unsafe {
        // as *mut _ change const to mut, _ is a type
        let usart1 = &mut *(stm32::USART1::ptr() as *mut _);
        (usart1, cp.ITM, mono_timer)
    }
}
