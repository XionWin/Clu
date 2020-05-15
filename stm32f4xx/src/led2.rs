use hal::{prelude::*, stm32};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    
    let gpioh = dp.GPIOH.split();
    let mut led = gpioh.ph12.into_push_pull_output();
    led.set_high().unwrap();


    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let span = 1000u32;
    loop {
        // On for 1s, off for 1s.
        led.set_low().unwrap();
        delay.delay_ms(span);
        led.set_high().unwrap();
        delay.delay_ms(span);
    }
}
