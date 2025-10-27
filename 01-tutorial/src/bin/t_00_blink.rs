#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    info!("blink program started");

    // Configure PF9 and PF10 as push-pull outputs, initial High = LED off
    let mut led0 = Output::new(p.PF9, Level::High, Speed::High);
    let mut led1 = Output::new(p.PF10, Level::High, Speed::High);

    loop {
        // LED0 on (drive low), LED1 off (drive high)
        led0.set_low();
        led1.set_high();
        Timer::after(Duration::from_millis(500)).await;

        // LED0 off, LED1 on
        led0.set_high();
        led1.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
