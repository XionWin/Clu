use hal::{prelude::*, stm32};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    let gpiod = dp.GPIOD.split();
    let mut red = gpiod.pd0.into_push_pull_output();
    red.set_high().unwrap();
    let mut green = gpiod.pd2.into_push_pull_output();
    green.set_high().unwrap();
    let mut blue = gpiod.pd4.into_push_pull_output();
    blue.set_high().unwrap();


    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let span = 50u32;
    loop {
        // On for 1s, off for 1s.
        red.set_low().unwrap();
        green.set_high().unwrap();
        blue.set_high().unwrap();
        delay.delay_us(span);
        red.set_high().unwrap();
        green.set_low().unwrap();
        blue.set_high().unwrap();
        delay.delay_us(span);
        red.set_high().unwrap();
        green.set_high().unwrap();
        blue.set_low().unwrap();
        delay.delay_us(span);
    }
}
