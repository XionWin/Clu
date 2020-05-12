use hal::{
    prelude::*,
};

type Default = hal::spi::Spi<
    hal::stm32::SPI3,
    (
        hal::gpio::gpioc::PC10<hal::gpio::Alternate<hal::gpio::AF6>>,
        hal::gpio::gpioc::PC11<hal::gpio::Alternate<hal::gpio::AF6>>,
        hal::gpio::gpioc::PC12<hal::gpio::Alternate<hal::gpio::AF6>>,
    ),
>;

type CS = hal::gpio::gpiog::PG6<hal::gpio::Output<hal::gpio::PushPull>>;

pub struct SPI3 {
    writer: Default,
    cs: CS,
}

impl SPI3 {
    pub fn default(spi3: Default, cs: CS) -> Self {
        SPI3 { writer: spi3, cs }
    }
}

impl vd::bus::Bus for SPI3 {
    fn select(&mut self) {
        self.cs.set_low().unwrap();
    }
    fn write_byte(&mut self, _: u8, _: u8) {
        todo!()
    }
    fn read_byte(&mut self, address: u8) -> u8 {
        let mut buffer: [u8; 2] = [0x00; 2];
        buffer[0] = address;
        self.select();
        self.writer.transfer(&mut buffer).unwrap();
        self.deselect();
        buffer[1]
    }
    fn deselect(&mut self) {
        self.cs.set_high().unwrap();
    }
}
