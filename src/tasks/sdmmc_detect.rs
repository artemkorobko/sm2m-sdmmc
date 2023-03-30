use rtic::mutex_prelude::*;
use stm32f1xx_hal::timer::Event;

use crate::{app, state::AppState};

pub fn sdmmc_detect(mut cx: app::sdmmc_detect::Context) {
    let is_sdmmc_attached = cx.local.sdmmc_detect_pin.is_high();
    let detached_indicator = cx.local.sdmmc_detached_led;

    cx.shared.state.lock(|state| {
        if *state == AppState::NotReady && is_sdmmc_attached {
            detached_indicator.set_high();
            *state = AppState::Ready;
        } else if *state != AppState::NotReady && !is_sdmmc_attached {
            detached_indicator.set_low();
            *state = AppState::NotReady;
        }
    });

    cx.local.timer.clear_interrupt(Event::Update);
}
