#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
});

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let mut usart = Uart::new_blocking(p.USART1, p.PA10, p.PA9, config).unwrap();

    unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
    info!("wrote Hello, starting echo");

    // 单字节读取，避免溢出的方法是改用异步或DMA
    // 这里用最简单的方法：逐字节读取但加入错误处理
    let mut buf = [0u8; 1];
    loop {
        match usart.blocking_read(&mut buf) {
            Ok(_) => {
                // 读取成功，回显接收到的数据
                info!("Received: {:?}", buf[0] as char);
                match usart.blocking_write(&buf) {
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