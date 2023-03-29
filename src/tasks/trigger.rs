use stm32f1xx_hal::gpio::ExtiPin;

use crate::app;

pub fn trigger(cx: app::trigger::Context) {
    cx.local.trigger.clear_interrupt_pending_bit();
}
