#![no_std]

use stm32f407;
pub fn run() -> ! {
    // stm32f407::gpio::run();
    // stm32f407::led::run();
    // stm32f407::pwm::run();
    // stm32f407::spi::run();
    // stm32f407::i2c::run();
    // stm32f407::i2cpwm::run();
    // stm32f407::dual_i2c::run();
    // stm32f407::spi_i2c::run();
    // stm32f407::_spi::run();
    // stm32f407::uart::run();

    stm32f407::led2::run();

}