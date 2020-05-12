// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // stm32f4xx::gpio::run();
    // stm32f4xx::led::run();
    // stm32f4xx::pwm::run();
    // stm32f4xx::spi::run();
    // stm32f4xx::i2c::run();
    // stm32f4xx::i2cpwm::run();
    // stm32f4xx::dual_i2c::run();
    // stm32f4xx::spi_i2c::run();
    stm32f4xx::_spi::run();

    // stm32f4xx::led2::run();
}