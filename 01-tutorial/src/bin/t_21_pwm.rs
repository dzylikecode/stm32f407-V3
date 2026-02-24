#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const ARR_C: u32 = 500 - 1; // C 里用的 arr = 500-1
const PWM_HZ: u32 = 2; // 约 2kHz（84MHz/(84*500)=2000Hz）
const STEP_MAX: u32 = 300; // C 里 ledrpwmval 0..300
const ACTIVE_LOW: bool = true; // 对应 TIM_OCPOLARITY_LOW（若你的LED低电平亮就保持true）

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("start");

    // 对应：PF9 复用为 TIM14_CH1 (AF9)
    // Embassy 选对引脚外设后，会自动配置成正确的AF。
    let ch1_pin = PwmPin::new(p.PF9, OutputType::PushPull);

    // 对应：TIM14 + CH1 + 2kHz
    let mut pwm = SimplePwm::new(
        p.TIM14,
        Some(ch1_pin),
        None,
        None,
        None,
        khz(PWM_HZ),
        Default::default(),
    );

    let mut ch1 = pwm.ch1();
    ch1.enable();

    let max = ch1.max_duty_cycle();
    info!("pwm max duty = {}", max);

    // C: uint16_t ledrpwmval = 0; uint8_t dir = 1;
    let mut val: u32 = 0;
    let mut inc = true;

    loop {
        // C: if (ledrpwmval > 300) dir=0; if (ledrpwmval==0) dir=1;
        if val > STEP_MAX {
            inc = false;
        }
        if val == 0 {
            inc = true;
        }

        // 把 C 里的 0..ARR_C 映射到 Embassy 的 0..max
        // （因为 Embassy 为了凑频率，内部的 period 不一定刚好是 499）
        let mut duty = (max as u64 * val as u64 / ARR_C as u64) as u32;

        // C: TIM_OCPOLARITY_LOW（低有效）的话，占空比需要反一下才“val越大越亮”
        if ACTIVE_LOW {
            duty = max.saturating_sub(duty);
        }

        ch1.set_duty_cycle(duty);

        // C: delay_ms(10);
        Timer::after_millis(10).await;

        // C: dir ? ++ : --
        if inc {
            val = val.saturating_add(1);
        } else {
            val = val.saturating_sub(1);
        }
    }
}
