use hal::{serial::{Serial, config::Config}, prelude::*, stm32};


pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(128.mhz())
        .freeze();

    let gpiob = dp.GPIOB.split();
    let gpiod = dp.GPIOD.split();
    let gpioc = dp.GPIOC.split();
    let tx = gpiod.pd8.into_alternate_af7();
    let rx = gpiob.pb11.into_alternate_af7();
    let serial_3 = Serial::usart3(
        dp.USART3, 
        (tx, rx), 
        Config::default().baudrate(115_200.bps()),
        clocks
    ).unwrap();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap();

    let (mut tx, mut rx) = serial_3.split();

    let sent = 0x08;

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`

    loop {

        block!(tx.write(sent)).ok();

        let received = rx.read();

        if received.is_ok() && received.unwrap() == sent {
            led.set_low().unwrap();
            delay.delay_ms(100u32);
            led.set_high().unwrap();
            delay.delay_ms(100u32);
        } else {
            led.set_high().unwrap();
            delay.delay_ms(1000u32);
        }

    }

    // loop {
    //     led.set_low().unwrap();
    //     delay.delay_ms(1000u32);
    // }
}
