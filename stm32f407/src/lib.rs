#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32f4xx_hal as hal;
extern crate vdhal as vd;
#[macro_use(block)]
extern crate nb;

pub mod gpio;
pub mod led;
pub mod led2;
pub mod pwm;
pub mod spi;
pub mod i2c;
pub mod i2cpwm;
pub mod dual_i2c;
pub mod spi_i2c;

pub mod uart;
pub mod uart2;

pub mod _spi;