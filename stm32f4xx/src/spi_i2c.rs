use hal::{
    i2c::I2c,
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    stm32,
};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split();
    let gpiog = dp.GPIOG.split();
    let gpiof = dp.GPIOF.split();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(168.mhz())
        .pclk1(24.mhz())
        .freeze();

    let mut spi_3 = Spi::spi3(
        dp.SPI3,
        (
            gpioc.pc10.into_alternate_af6().internal_pull_up(true),
            gpioc.pc11.into_alternate_af6().internal_pull_up(true),
            gpioc.pc12.into_alternate_af6().internal_pull_up(true),
        ),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        16_000_000.hz(),
        clocks,
    );

    let mut i2c_2 = I2c::i2c2(
        dp.I2C2,
        (
            gpiof.pf1.into_alternate_af4().set_open_drain(),
            gpiof.pf0.into_alternate_af4().set_open_drain(),
        ),
        400.khz(),
        clocks,
    );

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    let mut cs = gpiog.pg6.into_push_pull_output();
    cs.set_high().unwrap();

    // Create an `u8` array, which can be transfered via SPI.
    let mut msg_send: [u8; 2] = [0x0; 2];
    let mut spi_received: [u8; 2] = [0x0; 2];
    let mut i2c_received: [u8; 1] = [0x0; 1];
    let mut flag = true;

    loop {
        led.set_low().unwrap();

        for _ in 0..10000 {
            msg_send[0] = 0x75 | 0x80;
            let mut msg_sending = msg_send.clone();
            cs.set_low().unwrap();
            let data = spi_3.transfer(&mut msg_sending).ok();
            cs.set_high().unwrap();
            spi_received.clone_from_slice(data.unwrap());
            if spi_received[1] != 0x70 {
                flag = false;
            }
        }
        led.set_high().unwrap();
        delay.delay_ms(1000u32);
        led.set_low().unwrap();

        for _ in 0..10000 {
            i2c_2.write_read(0x68, &[0x75], &mut i2c_received).unwrap();
            if i2c_received[0] != 0x70 {
                flag = false;
            }
        }

        led.set_high().unwrap();
        delay.delay_ms(1000u32);

        // Check, if msg_send and msg_received are identical.
        // This succeeds, when master and slave of the SPI are connected.
        // assert_eq!(msg_send, msg_received);
        if !flag {
            led.set_low().unwrap();
            break;
        }
    }

    loop {}
}
