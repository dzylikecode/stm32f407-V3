#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART1, p.PA10, p.PA9, Irqs, p.DMA2_CH7, p.DMA2_CH2, config,
    )
    .unwrap();

    info!("wrote Hello, starting echo");

    // 异步接收，高效处理
    let mut buf = [0u8; 32];
    loop {
        match usart.read(&mut buf).await {
            Ok(_) => {
                // 读取成功，回显接收到的数据
                info!("Received: {:?}", buf);
                match usart.write(&buf).await {
                    Ok(_) => {},
                    Err(e) => defmt::warn!("Write error: {:?}", e),
                }
            }
            Err(e) => {
                // 打印错误但不停止，清除错误继续
                defmt::warn!("Read error: {:?}", e);
            }
        }
    }
}
