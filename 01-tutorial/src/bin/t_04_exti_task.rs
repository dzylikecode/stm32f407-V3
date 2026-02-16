#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::Pull;
use embassy_stm32::{bind_interrupts, interrupt};
use {defmt_rtt as _, panic_probe as _};
use embassy_sync::channel::{Channel, Sender};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;

bind_interrupts!(
    pub struct Irqs{
        EXTI0 => exti::InterruptHandler<interrupt::typelevel::EXTI0>;
        EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
        EXTI3 => exti::InterruptHandler<interrupt::typelevel::EXTI3>;
        EXTI4 => exti::InterruptHandler<interrupt::typelevel::EXTI4>;
});

#[derive(Clone, Copy)]
enum KeyEvent {
  Up,
  Key1,
  Key2,
  Key3,
}

static CHANNEL: Channel<ThreadModeRawMutex, KeyEvent, 64> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Start exti!");

    let key_up = ExtiInput::new(p.PA0, p.EXTI0, Pull::Up, Irqs);
    let key1 = ExtiInput::new(p.PE2, p.EXTI2, Pull::Up, Irqs);
    let key2 = ExtiInput::new(p.PE3, p.EXTI3, Pull::Up, Irqs);
    let key3 = ExtiInput::new(p.PE4, p.EXTI4, Pull::Up, Irqs);  

    info!("Press the USER button...");

    spawner.spawn(key_task(CHANNEL.sender(), key_up, KeyEvent::Up)).unwrap();
    spawner.spawn(key_task(CHANNEL.sender(), key1, KeyEvent::Key1)).unwrap();
    spawner.spawn(key_task(CHANNEL.sender(), key2, KeyEvent::Key2)).unwrap();
    spawner.spawn(key_task(CHANNEL.sender(), key3, KeyEvent::Key3)).unwrap();

    loop {
        match CHANNEL.receive().await {
            KeyEvent::Up => info!("Up"),
            KeyEvent::Key1 => info!("Key1"),
            KeyEvent::Key2 => info!("Key2"),
            KeyEvent::Key3 => info!("Key3"),
        }
    }
}

#[embassy_executor::task(pool_size = 4)]
async fn key_task(
  control: Sender<'static, ThreadModeRawMutex, KeyEvent, 64>,
  mut key: ExtiInput<'static>, key_event: KeyEvent) {
    loop {
        key.wait_for_falling_edge().await;
        control.send(key_event).await;
    }
}

