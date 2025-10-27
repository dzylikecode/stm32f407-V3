#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use tutorial::peripherals::led::Led;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    info!("blink program started");
    let mut led0 = Led::new(p.PF9, true);
    let mut led1 = Led::new(p.PF10, true);

    loop {
        led0.enable();
        led1.disable();
        Timer::after(Duration::from_millis(500)).await;

        led0.disable();
        led1.enable();
        Timer::after(Duration::from_millis(500)).await;
    }
}
