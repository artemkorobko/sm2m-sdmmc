use embedded_sdmmc::Mode;
use rtic::Mutex;
use stm32f1xx_hal::gpio::ExtiPin;

use crate::{
    app,
    peripherals::{self, led},
};

// pub fn exti0(mut cx: app::exti0::Context) {
//     // cortex_m_semihosting::hprintln!("exti0");

//     let sd_card_detect_pin = cx.local.sd_card_detect_pin;
//     let sd_card_detect_led_pin = cx.local.sd_card_detect_led_pin;

//     let state = read_sd_card_state(sd_card_detect_pin);
//     sd_card_detect_pin.clear_interrupt_pending_bit();

//     display_sd_card_state(&state, sd_card_detect_led_pin);

//     // cx.shared.sd_card_state.lock(|sd_card_state| {
//     //     *sd_card_state = state;
//     // });

//     // cx.shared.status_led_pin.lock(|pin| {
//     //     display_sd_card_state(&state, pin);
//     // });

//     // if state == peripherals::SdCardState::Connected {
//     //     cx.shared.sdmmc_controller.lock(|controller| {
//     //         if controller.device().init().is_ok() {
//     //             if let Ok(mut volume) = controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
//     //                 if let Ok(dir) = controller.open_root_dir(&volume) {
//     //                     if let Ok(mut file) = controller.open_file_in_dir(
//     //                         &mut volume,
//     //                         &dir,
//     //                         "example.txt",
//     //                         Mode::ReadWriteCreateOrTruncate,
//     //                     ) {
//     //                         if let Err(_) =
//     //                             controller.write(&mut volume, &mut file, b"testing file writes.")
//     //                         {
//     //                             cx.shared.status_led_pin.lock(|pin| led::on(pin));
//     //                         }
//     //                         controller.close_file(&volume, file).ok();
//     //                         controller.close_dir(&volume, dir);
//     //                     }
//     //                 }
//     //             }
//     //         }
//     //     });
//     // }
// }

// pub fn display_sd_card_state(state: &peripherals::SdCardState, pin: &mut app::SdCardDetectLedPin) {
//     match state {
//         peripherals::SdCardState::Disconnected => led::on(pin),
//         peripherals::SdCardState::Connected => led::off(pin),
//     }
// }
