use stm32f1xx_hal::gpio::ExtiPin;

use crate::app;

pub fn trigger(cx: app::trigger::Context) {
    cx.local.trigger.clear_interrupt_pending_bit();
}

// fn write(bus: &mut SdCard, file: u16, buffer: Buffer) {
//     // read from SDMMC to BUS
//     // let chunk_size = 2;
//     // for chunk in buffer.chunks(chunk_size) {
//     //     if chunk.len() < chunk_size {
//     //         let word = chunk[0] as u16;
//     //     } else {
//     //         let word = chunk[0] as u16 | (chunk[1] as u16) << 8;
//     //     }
//     // }

//     if let Ok(sdmmc_block) = bus.acquire() {
//         let time_source = StaticTimeSource::default();
//         let mut controller = embedded_sdmmc::Controller::new(sdmmc_block, time_source);
//         if let Ok(mut volume) = controller.get_volume(embedded_sdmmc::VolumeIdx(0)) {
//             if let Ok(dir) = controller.open_root_dir(&volume) {
//                 let file_name = file.as_file_name();
//                 let file_mode = embedded_sdmmc::Mode::ReadWriteCreateOrTruncate;
//                 if let Ok(mut file) =
//                     controller.open_file_in_dir(&mut volume, &dir, file_name.as_str(), file_mode)
//                 {
//                     if controller
//                         .write(&mut volume, &mut file, buffer.as_slice())
//                         .is_ok()
//                     {
//                         // Write completed
//                     } else {
//                         // Unablee to write to file
//                     }
//                     controller.close_file(&volume, file).ok();
//                     controller.close_dir(&volume, dir);
//                 } else {
//                     // Unable to open file
//                 }
//             } else {
//                 // SDMMC does not have a root directory
//             }
//         } else {
//             // SDMMC has no volumes
//         }
//     } else {
//         // SDMMC is not accessible
//     }
// }

// // fn write_to<D, T>(
// //     controller: &mut embedded_sdmmc::Controller<D, T>,
// //     volume: &mut embedded_sdmmc::Volume,
// //     file: &mut embedded_sdmmc::File,
// //     buffer: &BlockBuffer,
// // ) -> Result<usize, embedded_sdmmc::Error<<D as embedded_sdmmc::BlockDevice>::Error>>
// // where
// //     D: embedded_sdmmc::BlockDevice,
// //     T: embedded_sdmmc::TimeSource,
// // {
// //     controller.write(volume, file, buffer.as_slice())
// // }
