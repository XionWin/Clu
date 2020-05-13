use hal::pwm;
use hal::{prelude::*, stm32};

pub fn run() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz()) //discovery board has 8 MHz crystal for HSE
        .sysclk(128.mhz())
        .freeze();
        
    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpioa = dp.GPIOA.split();
    let channels = (
        gpioa.pa0.into_alternate_af2(),
        gpioa.pa1.into_alternate_af2(),
        gpioa.pa2.into_alternate_af2(),
        gpioa.pa3.into_alternate_af2(),
    );

    let pwm = pwm::tim5(dp.TIM5, channels, clocks, 20u32.khz());
    let (mut ch0, mut ch1, mut ch2, mut ch3) = pwm;
    let max_duty = ch1.get_max_duty();
    
    ch0.set_duty(max_duty);
    ch0.enable();
    ch1.set_duty(max_duty);
    ch1.enable();
    ch2.set_duty(max_duty);
    ch2.enable();
    ch3.set_duty(max_duty);
    ch3.enable();

    loop {
        // cortex_m::asm::nop();
        for value in 0..max_duty {
            ch0.set_duty(value);
            ch1.set_duty(value);
            ch2.set_duty(value);
            ch3.set_duty(value);
            delay.delay_us(100u32);
        }
    }
}
