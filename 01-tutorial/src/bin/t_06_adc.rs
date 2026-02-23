#![no_std]
#![no_main]

use cortex_m::singleton;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Peripherals;
use embassy_stm32::adc::{Adc, AdcChannel, RegularConversionMode, RingBufferedAdc, SampleTime};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

const ADC_MAX: f32 = 4095.0; // 12-bit
const VREF: f32 = 3.3; // 你 C 代码里用的 3.3V
const DMA_BUF_SIZE: usize = 50; // 对应你 C 里的 ADC_DMA_BUF_SIZE=50

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    spawner.spawn(adc_dma_task(p)).unwrap();

    // main 不做事也行
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn adc_dma_task(p: Peripherals) {
    // RingBufferedAdc::read 要求 out.len() == ring.len()/2
    const RING: usize = DMA_BUF_SIZE * 2;
    let ring: &mut [u16; RING] = singleton!(ADC_RING: [u16; RING] = [0; RING]).unwrap();

    // 每次我们“取一块”的输出缓冲：等价你 C 的 g_adc_dma_buf[50]
    let mut out = [0u16; DMA_BUF_SIZE];

    // 1) ADC 初始化（等价 adc_init + adc_dma_init 里的 ADC 部分）
    let adc = Adc::new_with_config(p.ADC1, Default::default());

    // 2) 配置 ADC + DMA（单通道）
    //    channels_iter 里只放一个通道，就等价你的单通道 DMA
    let mut adc: RingBufferedAdc<embassy_stm32::peripherals::ADC1> = adc.into_ring_buffered(
        p.DMA2_CH0, // ⚠️ 改成你 ADC1 对应的 DMA 通道（不一定是 CH0）
        ring,
        [
            // ⚠️ 改成你实际的 ADC1_CH5 引脚
            // 采样时间等价 ADC_SAMPLETIME_480CYCLES
            (p.PA5.degrade_adc(), SampleTime::CYCLES480),
        ]
        .into_iter(),
        // 连续转换 + DMA 连续搬运（等价你 C 里 CR2.CONT=1）
        RegularConversionMode::Continuous,
    );

    adc.start();
    info!("ADC DMA started");

    loop {
        // === 等价 adc_dma_enable(ADC_DMA_BUF_SIZE) + 等待 DMA 完成 ===
        match adc.read(&mut out).await {
            Ok(n) => {
                // 一般 n 会等于 DMA_BUF_SIZE
                let data = &out[..n];

                // === 等价 C 里 sum += g_adc_dma_buf[i]; adc = sum / BUF_SIZE; ===
                let mut sum: u32 = 0;
                for &v in data {
                    sum += v as u32;
                }
                let avg_raw: u16 = (sum / (data.len() as u32)) as u16;

                // === 等价 temp = adc * (3.3 / 4096) ===
                let voltage = (avg_raw as f32) * (VREF / ADC_MAX);

                info!(
                    "DMA block n={} avg_raw={} voltage={} V head={}",
                    n,
                    avg_raw,
                    voltage,
                    // 打印前几个样本看看波动（可选）
                    &data[..core::cmp::min(10, data.len())]
                );
            }
            Err(e) => {
                // overrun 或其它错误时，重启 DMA/ADC
                warn!("ADC DMA read error: {:?}", e);
                adc.start();
            }
        }

        // 等价你 C 里 delay_ms(100)
        Timer::after(Duration::from_millis(100)).await;
    }
}
