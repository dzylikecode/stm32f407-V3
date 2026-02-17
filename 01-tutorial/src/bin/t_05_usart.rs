#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart, dma};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let usart = Uart::new(
        p.USART1, p.PA10, p.PA9, Irqs, p.DMA2_CH7, p.DMA2_CH5, config,
    )
    .unwrap();
    let (mut tx, mut rx) = usart.split();

    unwrap!(tx.write(b"Hello Embassy World!\r\n").await);
    info!("wrote Hello, starting echo");

    // 异步接收，高效处理
    let mut buf = [0u8; 1];
    loop {
        match rx.read(&mut buf).await {
            Ok(_) => {
                info!("Received: {:?}", &buf);
                match tx.write(&buf).await {
                    Ok(_) => {}
                    Err(e) => defmt::warn!("Write error: {:?}", e),
                }
            }
            Err(e) => {
                defmt::warn!("Read error: {:?}", e);
            }
        }
    }
}
