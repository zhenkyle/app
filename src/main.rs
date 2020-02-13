#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f3xx_hal as hal;

use cortex_m_rt::{ExceptionFrame, entry, exception};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle as Rect};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::style::PrimitiveStyleBuilder;
use hal::i2c::{I2c as BlockingI2c};
use hal::prelude::*;
use hal::stm32;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    disp.init().unwrap();
    disp.flush().unwrap();

    Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
        .into_styled(style)
        .into_iter().draw(&mut disp);


    Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .into_iter().draw(&mut disp);


    Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
        .into_styled(style)
        .into_iter().draw(&mut disp);


    Rect::new(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
        .into_styled(style)
        .into_iter().draw(&mut disp);



    Circle::new(Point::new(96, 16 + 8), 8)
        .into_styled(style)
        .into_iter().draw(&mut disp);


    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

