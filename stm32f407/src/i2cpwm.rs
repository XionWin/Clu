use hal::{i2c::I2c, prelude::*, stm32, pwm};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(168.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();
    let scl = gpioa.pa8.into_alternate_af4().set_open_drain();
    let sda = gpioc.pc9.into_alternate_af4().set_open_drain();
    let mut i2c = I2c::i2c3(dp.I2C3, (scl, sda), 400.khz(), clocks);

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let channels = (
        gpioa.pa0.into_alternate_af2(),
        gpioa.pa1.into_alternate_af2(),
        gpioa.pa2.into_alternate_af2(),
        gpioa.pa3.into_alternate_af2(),
    );

    let pwm = pwm::tim5(dp.TIM5, channels, clocks, 20u32.khz());
    let (mut ch0, mut ch1, mut ch2, mut ch3) = pwm;
    let max_duty = ch0.get_max_duty();
    ch0.set_duty(max_duty);
    ch0.enable();
    ch1.set_duty(max_duty);
    ch1.enable();
    ch2.set_duty(max_duty);
    ch2.enable();
    ch3.set_duty(max_duty);
    ch3.enable();

    // Create an `u8` array, which can be transfered via SPI.
    // let mut msg_send: [u8; 2] = [0x0; 2];
    let mut msg_received: [u8; 1] = [0x0; 1];
    let mut flag = true;
    let mut direction = true;
    loop {
        // Clone the array, as it would be mutually shared in `transfer` while simultaniously would be
        // immutable shared in `assert_eq`.
        // let msg_sending = msg_send.clone();
        // Transfer the content of the array via SPI and receive it's output.
        // When MOSI and MISO pins are connected together, `msg_received` should receive the content.
        // from `msg_sending`
        // i2c.write(0x68, &[0x75]).unwrap();
        // i2c.read(0x68, &mut msg_received) .unwrap();

        for _ in 0..10000 {
            i2c.write_read(0x68, &[0x75], &mut msg_received).unwrap();
            if msg_received[0] != 0x71 {
                flag = false;
            }
        }

        // msg_received.clone_from_slice(data);

        // Check, if msg_send and msg_received are identical.
        // This succeeds, when master and slave of the SPI are connected.
        // assert_eq!(msg_send, msg_received);
        if !flag {
            break;
        }

        if direction {
            for value in max_duty - 100..max_duty + 1 {
                ch0.set_duty(value);
                ch1.set_duty(value);
                ch2.set_duty(value);
                delay.delay_us(5000u32);
            }
        } else {
            for value in max_duty - 100..max_duty + 1 {
                ch0.set_duty(2 * max_duty - value - 100);
                ch1.set_duty(2 * max_duty - value - 100);
                ch2.set_duty(2 * max_duty - value - 100);
                delay.delay_us(500u32);
            }
        }
        direction = !direction;
    }

    loop {
        ch0.set_duty(0);
        ch1.set_duty(0);
        ch2.set_duty(0);
        delay.delay_ms(1000u32);
    }
}
