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

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze();

    let mut spi = Spi::spi3(
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
        800_000.hz(),
        clocks,
    );


    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpiod = dp.GPIOD.split();
    let mut red = gpiod.pd0.into_push_pull_output();
    red.set_high().unwrap();
    let mut green = gpiod.pd2.into_push_pull_output();
    green.set_high().unwrap();
    let mut blue = gpiod.pd4.into_push_pull_output();
    blue.set_high().unwrap();

    // Create an `u8` array, which can be transfered via SPI.
    let mut msg_send: [u8; 32] = [0x0; 32];
    let mut msg_received: [u8; 32] = [0x0; 32];
    loop {
        msg_send[0] = 0x68;
        msg_send[1] = 0x75;
        // Clone the array, as it would be mutually shared in `transfer` while simultaniously would be
        // immutable shared in `assert_eq`.
        let mut msg_sending = msg_send.clone();
        // Transfer the content of the array via SPI and receive it's output.
        // When MOSI and MISO pins are connected together, `msg_received` should receive the content.
        // from `msg_sending`
        let data = spi.transfer(&mut msg_sending).unwrap();

        msg_received.clone_from_slice(data);

        // Check, if msg_send and msg_received are identical.
        // This succeeds, when master and slave of the SPI are connected.
        // assert_eq!(msg_send, msg_received);
        green.set_low().unwrap();
        delay.delay_ms(1000u32);
        green.set_high().unwrap();
        delay.delay_ms(1000u32);
    }
}
