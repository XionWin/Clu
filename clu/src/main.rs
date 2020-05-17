// std and main are not available for bare metal software
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;

use cortex_m_rt::entry;

use robomaster;
// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    robomaster::run();
}