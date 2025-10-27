use embassy_hal_internal::Peri;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
pub struct Led<'d> {
    output: Output<'d>,
}

impl<'d> Led<'d> {
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, enable: bool) -> Self {
        let output = Output::new(
            pin,
            if enable { Level::Low } else { Level::High },
            Speed::High,
        );
        Self { output }
    }

    #[inline]
    pub fn enable(&mut self) {
        self.output.set_low();
    }
    #[inline]
    pub fn disable(&mut self) {
        self.output.set_high();
    }
}
