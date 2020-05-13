use hal::{
    prelude::_embedded_hal_blocking_spi_Transfer,
};

// type Default = hal::spi::Spi<
//     hal::stm32::SPI3,
//     (
//         hal::gpio::gpioc::PC10<hal::gpio::Alternate<hal::gpio::AF6>>,
//         hal::gpio::gpioc::PC11<hal::gpio::Alternate<hal::gpio::AF6>>,
//         hal::gpio::gpioc::PC12<hal::gpio::Alternate<hal::gpio::AF6>>,
//     ),
// >;

// type CS = hal::gpio::gpiog::PG6<hal::gpio::Output<hal::gpio::PushPull>>;

pub struct SPI<S, C> 
where S: embedded_hal::blocking::spi::Transfer<u8>,
C: embedded_hal::digital::v2::OutputPin
{
    writer: S,
    cs: C,
}

impl<S, C> SPI<S, C> 
where S: embedded_hal::blocking::spi::Transfer<u8>,
C: embedded_hal::digital::v2::OutputPin
{
    pub fn default(spi: S, cs: C) -> Self {
        SPI { writer: spi, cs }
    }
}

impl<S, C> vd::bus::Bus for SPI<S, C>
where S: embedded_hal::blocking::spi::transfer::Default<u8>,
C: embedded_hal::digital::v2::OutputPin
{
    fn select(&mut self) {
        self.cs.set_low().ok();
    }
    fn write_byte(&mut self, _: u8, _: u8) {
        todo!()
    }
    fn read_byte(&mut self, address: u8) -> u8 {
        let mut buffer: [u8; 4] = [0x00; 4];
        buffer[0] = address;
        self.select();
        self.writer.transfer(&mut buffer).ok();
        self.deselect();
        buffer[1]
    }
    fn deselect(&mut self) {
        self.cs.set_high().ok();
    }
}
