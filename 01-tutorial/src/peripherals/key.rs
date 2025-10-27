use embassy_hal_internal::Peri;
use embassy_stm32::gpio::{Input, Pin, Pull, Speed};
pub struct Key<'d> {
    input: Input<'d>,
}

impl<'d> Key<'d> {
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, initial_value: Pull) -> Self {
        let input = Input::new(pin, initial_value);
        Self { input }
    }

    #[inline]
    pub fn is_up(&self) -> bool {
        self.input.is_high()
    }
    #[inline]
    pub fn is_down(&self) -> bool {
        self.input.is_low()
    }
}
