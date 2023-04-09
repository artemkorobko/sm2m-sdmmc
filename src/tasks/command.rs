use stm32f1xx_hal::gpio::ExtiPin;

use crate::{
    app,
    error::AppError,
    mode::Mode,
    peripherals::sm2m::{self, Read, Write},
};

use super::{mode_address, mode_read, mode_ready, mode_write};

pub enum Complete {
    Continue,
    Mode(Mode),
    Reply(u16),
    ReplyMode(u16, Mode),
}

pub fn command(cx: app::command::Context) {
    let mode = cx.local.state;
    let error_led = cx.local.error_led;
    let write_led = cx.local.write_led;
    let read_led = cx.local.read_led;
    let sdmmc_detect = cx.local.sdmmc_detect_pin;
    let card = cx.local.card;
    let input = unsafe { cx.local.input_bus.read() };
    let out_bus = cx.local.output_bus;
    let result = match mode {
        Mode::Ready => mode_ready::handle(input, error_led, write_led, read_led, sdmmc_detect),
        Mode::Address(file) => mode_address::handle(input, write_led, read_led, card, file),
        Mode::Write(file, buf) => mode_write::handle(input, file, buf, write_led, card),
        Mode::Read(file, buf, pos) => {
            mode_read::handle(input, file, buf, pos, read_led, card, out_bus)
        }
        Mode::Error(err) => mode_error(err, out_bus),
    };

    match result {
        Ok(Complete::Mode(m)) => {
            *mode = m;
            send_confirmation(out_bus);
        }
        Ok(Complete::Continue) => send_confirmation(out_bus),
        Ok(Complete::Reply(data)) => send_data(data, out_bus),
        Ok(Complete::ReplyMode(data, m)) => {
            *mode = m;
            send_data(data, out_bus);
        }
        Err(err) => {
            error_led.set_low();
            unsafe { out_bus.write(&sm2m::Output::error()) };
            *mode = Mode::Error(err);
        }
    }

    cx.local.trigger.clear_interrupt_pending_bit();
}

fn mode_error<W: sm2m::Write>(err: &AppError, bus: &mut W) -> Result<Complete, AppError> {
    unsafe { bus.write(&sm2m::Output::data(err.opcode())) }
    Ok(Complete::Mode(Mode::Ready))
}

fn send_confirmation<W: sm2m::Write>(bus: &mut W) {
    unsafe { bus.write(&sm2m::Output::ok()) };
}

fn send_data<W: sm2m::Write>(data: u16, bus: &mut W) {
    unsafe { bus.write(&sm2m::Output::data(data)) }
}
