#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{BufferedUart, Config};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embedded_io_async::{BufRead, Write};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::BufferedInterruptHandler<peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let config = Config::default();

    let mut tx_buf = [0u8; 32];
    let mut rx_buf = [0u8; 32];
    let mut buf_usart = BufferedUart::new(p.USART1, p.PA10, p.PA9, &mut tx_buf, &mut rx_buf, Irqs, config).unwrap();

    loop {
        let buf = buf_usart.fill_buf().await.unwrap();
        let n = buf.len();
        
        // 将数据复制到临时缓冲区
        let mut temp_buf = [0u8; 32];
        temp_buf[..n].copy_from_slice(&buf[..n]);
        
        info!("Received: {}", &temp_buf[..n]);
        
        // 消费掉已读取的数据
        buf_usart.consume(n);
        
        // 发送回去
        buf_usart.write(&temp_buf[..n]).await.unwrap();
    }
}
