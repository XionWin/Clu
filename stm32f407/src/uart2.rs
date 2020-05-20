use hal::{
    prelude::*,
    serial::{config::Config, Serial},
    stm32,
};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        // .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        // .pclk1(8.mhz())
        // .pclk2(8.mhz())
        .sysclk(48.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioh = dp.GPIOH.split();
    let tx = gpioa.pa9.into_alternate_af7();
    let rx = gpiob.pb7.into_alternate_af7();
    // let serial_1 = Serial::usart1(
    //     dp.USART1,
    //     (tx, rx),
    //     Config::default().baudrate(115_200.bps()),
    //     clocks,
    // )
    // .unwrap();


      let serial_1 = Serial::usart1(
        dp.USART1,
        (tx, rx),
        Config::default().baudrate(9_600.bps()),
        clocks
    )
    .unwrap();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led = gpioh.ph12.into_push_pull_output();
    led.set_high().unwrap();

    let (mut tx, mut _rx) = serial_1.split();

    let sent = b"hello world! \r\n";

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`

    loop {
        for &b in sent {
            block!(tx.write(b)).ok();
        }
        led.set_low().unwrap();
        delay.delay_ms(100u32);
        led.set_high().unwrap();
        delay.delay_ms(100u32);




        // block!(tx.write(sent)).ok();

        // let received = rx.read();

        // if received.is_ok() && received.unwrap() == sent {
        //     led.set_low().unwrap();
        //     delay.delay_ms(100u32);
        //     led.set_high().unwrap();
        //     delay.delay_ms(100u32);
        // } else {
        //     led.set_high().unwrap();
        //     delay.delay_ms(1000u32);
        // }
    }

    // loop {
    //     // led.set_low().unwrap();
    //     // delay.delay_ms(1000u32);
    // }
}
