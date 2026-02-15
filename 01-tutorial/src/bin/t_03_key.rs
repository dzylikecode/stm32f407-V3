#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};
use tutorial::peripherals::beep::Beep;
use tutorial::peripherals::led::Led;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);
    let key1 = Input::new(p.PE2, Pull::Up);
    let key2 = Input::new(p.PE3, Pull::Up);
    let key3 = Input::new(p.PE4, Pull::Up);
    let key_up = Input::new(p.PA0, Pull::Up);

    info!("key program started");
    let mut led0 = Led::new(p.PF9, true);
    let mut led0_status = true;
    let mut led1 = Led::new(p.PF10, true);
    let mut led1_status = true;
    let mut beep = Beep::new(p.PF8, false);
    let mut beep_status = false;

    // 可以明显看到没有消抖
    loop {
        // \dot x = f(x, u)
        if key_up.is_low() {
            beep_status = !beep_status;
            info!("beep pressed");
        }
        if key1.is_low() {
            led0_status = !led0_status;
            info!("key1 pressed");
        }
        if key2.is_low() {
            led1_status = !led1_status;
            info!("key2 pressed");
        }
        if key3.is_low() {
            led0_status = !led0_status;
            led1_status = !led1_status;
            info!("key3 pressed");
        }

        // y = A x
        if beep_status {
            beep.enable();
        } else {
            beep.disable();
        }

        if led0_status {
            led0.enable();
        } else {
            led0.disable();
        }

        if led1_status {
            led1.enable();
        } else {
            led1.disable();
        }
    }
}
