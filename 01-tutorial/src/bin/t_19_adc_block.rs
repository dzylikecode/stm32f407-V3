#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, SampleTime, VrefInt};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

/// 12-bit ADC max value (4095)
const ADC_MAX: u32 = (1 << 12) - 1;

/// 读 times 次取平均（异步版本：每次采样之间用 Timer::after）
async fn adc_read_average_u16<P>(
    adc: &mut Adc<'static, embassy_stm32::peripherals::ADC1>,
    pin: &mut P,
    st: SampleTime,
    times: u8,
    gap_ms: u64,
) -> u16
where
    P: embassy_stm32::adc::AdcChannel<embassy_stm32::peripherals::ADC1>,
{
    let mut acc: u32 = 0;

    for _ in 0..times {
        let v = adc.blocking_read(pin, st) as u32;
        acc += v;

        // 等价于你 C 里的 delay_ms(5)，但这是 async 不阻塞
        Timer::after(Duration::from_millis(gap_ms)).await;
    }

    (acc / (times as u32)) as u16
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("adc demo start");

    let mut adc = Adc::new_with_config(p.ADC1, Default::default());

    // ⚠️ 改成你实际的 ADC 引脚
    let mut pin = p.PA5;

    let sample_time = SampleTime::CYCLES480;

    // === VrefInt（异步启动等待）===
    let mut vrefint = adc.enable_vrefint();

    // 原来是 delay_us(...)，现在改成 async Timer
    Timer::after(Duration::from_micros(VrefInt::start_time_us() as u64)).await;

    let vrefint_sample = adc.blocking_read(&mut vrefint, SampleTime::CYCLES112);

    const VREFINT_MV: u32 = 1210;

    let convert_to_millivolts =
        |sample: u16| -> u32 { (u32::from(sample) * VREFINT_MV) / u32::from(vrefint_sample) };

    let vdda_mv = convert_to_millivolts(ADC_MAX as u16);
    info!("vrefint_sample = {}", vrefint_sample);
    info!("estimated VDDA = {} mV", vdda_mv);

    loop {
        // 单次读取
        let once = adc.blocking_read(&mut pin, sample_time);
        info!("ADC once: {} ({} mV)", once, convert_to_millivolts(once));

        // 10次平均（每次间隔 5ms）
        let avg = adc_read_average_u16(&mut adc, &mut pin, sample_time, 10, 5).await;
        let avg_mv = convert_to_millivolts(avg);
        let avg_v = (avg_mv as f32) / 1000.0;
        info!("ADC avg(10): {} ({} mV, {} V)", avg, avg_mv, avg_v);

        Timer::after(Duration::from_millis(100)).await;
    }
}
