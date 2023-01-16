use rtic::mutex_prelude::*;
use stm32f1xx_hal::timer::Event;

use crate::{
    app,
    peripherals::sdmmc::{AsFileName, StaticTimeSource},
    types::sdmmc,
};

pub fn sdmmc_detect(cx: app::sdmmc_detect::Context) {
    let timer = cx.local.timer;
    let sdmmc_detect_led = cx.local.sdmmc_detect_led;
    let sdmmc_detect_pin = cx.local.sdmmc_detect_pin;
    let mut sdmmc_attached_flag = cx.shared.sdmmc_attached_flag;
    let is_sdmmc_attached = sdmmc_detect_pin.is_high();

    sdmmc_attached_flag.lock(|sdmmc_attached_flag| {
        *sdmmc_attached_flag = is_sdmmc_attached;
    });

    if is_sdmmc_attached {
        sdmmc_detect_led.off();
    } else {
        sdmmc_detect_led.on();
    }

    timer.clear_interrupt(Event::Update);
}

fn write(bus: &mut sdmmc::Bus, file: u16, buffer: sdmmc::Buffer) {
    if let Ok(sdmmc_block) = bus.acquire() {
        let time_source = StaticTimeSource::default();
        let mut controller = embedded_sdmmc::Controller::new(sdmmc_block, time_source);
        if let Ok(mut volume) = controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
            if let Ok(dir) = controller.open_root_dir(&volume) {
                let file_name = file.as_file_name();
                let file_mode = embedded_sdmmc::Mode::ReadWriteCreateOrTruncate;
                if let Ok(mut file) =
                    controller.open_file_in_dir(&mut volume, &dir, file_name.as_str(), file_mode)
                {
                    if controller
                        .write(&mut volume, &mut file, buffer.as_slice())
                        .is_ok()
                    {
                        // Write completed
                    } else {
                        // Unablee to write to file
                    }
                    controller.close_file(&volume, file).ok();
                    controller.close_dir(&volume, dir);
                } else {
                    // Unable to open file
                }
            } else {
                // SDMMC does not have a root directory
            }
        } else {
            // SDMMC has no volumes
        }
    } else {
        // SDMMC is not accessible
    }
}

// fn write_to<D, T>(
//     controller: &mut embedded_sdmmc::Controller<D, T>,
//     volume: &mut embedded_sdmmc::Volume,
//     file: &mut embedded_sdmmc::File,
//     buffer: &BlockBuffer,
// ) -> Result<usize, embedded_sdmmc::Error<<D as embedded_sdmmc::BlockDevice>::Error>>
// where
//     D: embedded_sdmmc::BlockDevice,
//     T: embedded_sdmmc::TimeSource,
// {
//     controller.write(volume, file, buffer.as_slice())
// }
