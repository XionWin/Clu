use hal::{
    prelude::*,
    spi::{Mode, Phase, Polarity, Spi},
    stm32,
};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
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
        8000_000.hz(),
        clocks,
    );

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    let mut cs = gpiod.pd4.into_push_pull_output();
    cs.set_high().unwrap();

    // Create an `u8` array, which can be transfered via SPI.
    let mut msg_send: [u8; 2] = [0x0; 2];
    let mut msg_received: [u8; 2] = [0x0; 2];
    loop {
        // msg_send[0] = 0x68;
        // msg_send[1] = (0x75 << 1) | 0x00;
        msg_send[0] = 0x75 | 0x80;
        // Clone the array, as it would be mutually shared in `transfer` while simultaniously would be
        // immutable shared in `assert_eq`.
        let mut msg_sending = msg_send.clone();
        // Transfer the content of the array via SPI and receive it's output.
        // When MOSI and MISO pins are connected together, `msg_received` should receive the content.
        // from `msspi_3ending`
        cs.set_low().unwrap();
        let data = spi_3.transfer(&mut msg_sending).ok();
        cs.set_high().unwrap();
        // spi.write(& [0x75]).unwrap();
        // let _b = spi.read().unwrap();

        msg_received.clone_from_slice(data.unwrap());

        // Check, if msg_send and msg_received are identical.
        // This succeeds, when master and slave of the SPI are connected.
        // assert_eq!(msg_send, msg_received);
        if msg_received[1] == 0x70 {
            led.set_low().unwrap();
            delay.delay_ms(1000u32);
            led.set_high().unwrap();
            delay.delay_ms(1000u32);
        }
    }
}
