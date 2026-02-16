#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::Pull;
use embassy_stm32::{bind_interrupts, interrupt};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(
    pub struct Irqs{
        EXTI0 => exti::InterruptHandler<interrupt::typelevel::EXTI0>;
        EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
        EXTI3 => exti::InterruptHandler<interrupt::typelevel::EXTI3>;
        EXTI4 => exti::InterruptHandler<interrupt::typelevel::EXTI4>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Start exti!");

    let mut key_up = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up, Irqs);
    let mut key1 = ExtiInput::new(p.PE2, p.EXTI2, Pull::Up, Irqs);
    let mut key2 = ExtiInput::new(p.PE3, p.EXTI3, Pull::Up, Irqs);
    let mut key3 = ExtiInput::new(p.PE4, p.EXTI4, Pull::Up, Irqs);  

    info!("Press the USER button...");

    loop {
        key1.wait_for_rising_edge().await;
        info!("Pressed!");
        key1.wait_for_falling_edge().await;
        info!("Released!");
    }
}