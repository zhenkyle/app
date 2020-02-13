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
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::prelude::*;
use hal::stm32;
use hal::delay::Delay;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    
    let mut delay = Delay::new(cp.SYST, clocks);
    
    // SPI
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // rst and dc PIN
    let mut rst = gpiob.pb0.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let dc = gpiob.pb1.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay).unwrap();

    disp.init().unwrap();
    disp.flush().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

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

