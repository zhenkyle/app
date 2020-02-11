#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_imports)]
use cortex_m::{ iprint, iprintln};

use cortex_m_rt::entry;

#[allow(unused_imports)]
use app::init;
//use f3::hal::hal::blocking::delay::DelayMs;
use f3::hal::prelude::*; // for DelayMs

// Slave address
const MAGNETOMETER: u8 = 0b001_1110;

// Address of magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm) = app::init();
    loop {
        // Stage 1: Send the address of register we want to read to the magnetometer
        i2c1.cr2.write(|w| w.nbytes().bits(1)            // Number of bits to read
                       .sadd().bits((MAGNETOMETER <<1) as u16)       // Slave address
                       .rd_wrn().clear_bit()                         // 1 Master request a write
                       .autoend().clear_bit()                        // We will send STOP manually
                       .start().set_bit()                            // Start, it seems i2c1.cr2.write will combine all
                                                                     // fields together, so no need for start to be the
                                                                     // last one
        );
    
        while i2c1.isr.read().txis().bit_is_clear() {};            // Wait until hardware tell us we can write I2C_TXDR
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));       // Send the address we want to read: IRA_REG_M

         while i2c1.isr.read().tc().bit_is_clear() {};         // Wailt until transfer is complete    

        // Stage 2: Receive the contents of the register we asked for
        i2c1.cr2.modify(|_, w| w.nbytes().bits(6)              // Number of bits to read
                       .rd_wrn().set_bit()                     // 0 Master request a read
                       .autoend().set_bit()                    // STOP automaticly
                       .start().set_bit());                    // START
    
    
        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            while i2c1.isr.read().rxne().bit_is_clear() {};       // Wailt until read is complete
            *byte = i2c1.rxdr.read().rxdata().bits();
        }
        let x_h = u16::from(buffer[0]);
        let x_l = u16::from(buffer[1]);
        let z_h = u16::from(buffer[2]);
        let z_l = u16::from(buffer[3]);
        let y_h = u16::from(buffer[4]);
        let y_l = u16::from(buffer[5]);

        let x = ((x_h << 8) + x_l) as i16;
        let y = ((y_h << 8) + y_l) as i16;
        let z = ((z_h << 8) + z_l) as i16;
        
        iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));
        delay.delay_ms(1_000_u16);
    }
}

