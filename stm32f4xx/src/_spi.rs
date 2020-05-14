use hal::{
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    stm32,
};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(128.mhz())
        .pclk1(24.mhz())
        .freeze();

    let _spi_2 = Spi::spi2(
        dp.SPI2,
        (
            gpiob.pb10.into_alternate_af5(),
            gpioc.pc2.into_alternate_af5(),
            gpioc.pc3.into_alternate_af5(),
        ),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16_000_000.hz(),
        clocks,
    );

    let spi_3 = Spi::spi3(
        dp.SPI3,
        (
            gpioc.pc10.into_alternate_af6(),
            gpioc.pc11.into_alternate_af6(),
            gpioc.pc12.into_alternate_af6(),
        ),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16_000_000.hz(),
        clocks,
    );

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    let mut cs = gpiog.pg6.into_push_pull_output();
    cs.set_high().unwrap();

    let spi_3: &mut dyn vd::bus::Bus = &mut crate::spi::SPI::default(spi_3, cs);
    let mut flag = true;
    loop {
        for _ in 0..10000 {
            if crate::_spi::read(spi_3) != 0x70 {
                flag = false;
            }
        }

        // Check, if msg_send and msg_received are identical.
        // This succeeds, when master and slave of the SPI are connected.
        // assert_eq!(msg_send, msg_received);
        if flag {
            led.set_low().unwrap();
            delay.delay_ms(1000u32);
            led.set_high().unwrap();
            delay.delay_ms(1000u32);
        } else {
            led.set_low().unwrap();
            break;
        }
    }

    loop {}
}

pub fn read(spi: &mut dyn vd::bus::Bus) -> u8 {
    spi.read_byte(0x75)
}
