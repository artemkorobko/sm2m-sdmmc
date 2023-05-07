// use stm32f1xx_hal::gpio::ExtiPin;

// use crate::{
//     app,
//     error::AppError,
//     mode::{address, read, ready, result::Complete, write, Mode},
//     peripherals::*,
// };

// pub fn command(cx: app::command::Context) {
//     defmt::info!("Inside SM2M interrupt");

//     let mode = cx.local.mode;
//     let card = cx.local.card;
//     let bus = cx.local.bus;
//     let sdmmc_detect = cx.local.sdmmc_detect_pin;
//     let indicators = cx.local.indicators;

//     // let input = unsafe { cx.local.input_bus.read() };
//     // let result = match mode {
//     //     Mode::Ready => ready::handle(input, error_led, write_led, read_led, sdmmc_detect),
//     //     Mode::Address(file) => address::handle(input, write_led, read_led, card, file),
//     //     Mode::Write(file, buf) => write::handle(input, write_led, file, buf, card),
//     //     Mode::Read(file, file_pos, buf, buf_pos) => {
//     //         read::handle(input, file, file_pos, buf, buf_pos, read_led, card)
//     //     }
//     //     Mode::Error(err) => mode_error(err, out_bus),
//     // };

//     // match result {
//     //     Ok(Complete::Mode(m)) => {
//     //         *mode = m;
//     //         send_confirmation(out_bus);
//     //     }
//     //     Ok(Complete::Continue) => send_confirmation(out_bus),
//     //     Ok(Complete::Reply(data)) => send_data(data, out_bus),
//     //     Err(err) => {
//     //         error_led.set_low(); // turn on error led
//     //         unsafe { out_bus.write(&sm2m::Output::error()) };
//     //         *mode = Mode::Error(err);
//     //     }
//     // }

//     cx.local.trigger.clear_interrupt_pending_bit();
// }

// fn mode_error<W: sm2m::Write>(err: &AppError, bus: &mut W) -> Result<Complete, AppError> {
//     unsafe { bus.write(&sm2m::Output::data(err.opcode())) }
//     Ok(Complete::Mode(Mode::Ready))
// }

// fn send_confirmation<W: sm2m::Write>(bus: &mut W) {
//     unsafe { bus.write(&sm2m::Output::ok()) };
// }

// fn send_data<W: sm2m::Write>(data: u16, bus: &mut W) {
//     unsafe { bus.write(&sm2m::Output::data(data)) }
// }
