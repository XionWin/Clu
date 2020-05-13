// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f407;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // stm32f407::gpio::run();
    // stm32f407::led::run();
    // stm32f407::pwm::run();
    // stm32f407::spi::run();
    // stm32f407::i2c::run();
    // stm32f407::i2cpwm::run();
    // stm32f407::dual_i2c::run();
    // stm32f407::spi_i2c::run();
    stm32f407::_spi::run();

    // stm32f407::led2::run();
}