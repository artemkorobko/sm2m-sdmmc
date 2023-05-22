use crate::{
    error::AppError,
    peripherals::{
        sdmmc,
        sm2m::{input, output},
        Indicators,
    },
};

enum Mode {
    Ready,
    Address,
    Read,
    Write,
    Error(u16),
}

const IO_BUFFER_SIZE: usize = 2 * 1024 * 5; // 2 bytes per word * 1024 bytes * 5 KB = 10 KB

pub struct Device {
    input: input::Bus,
    output: output::Bus,
    card: sdmmc::Card,
    indicators: Indicators,
    mode: Mode,
    file_name: sdmmc::FileName,
    buf: [u8; IO_BUFFER_SIZE],
    buf_pos: usize,
    file_pos: usize,
}

impl Device {
    pub fn new(
        input: input::Bus,
        output: output::Bus,
        card: sdmmc::Card,
        indicators: Indicators,
    ) -> Self {
        Self {
            input,
            output,
            card,
            indicators,
            mode: Mode::Ready,
            file_name: sdmmc::FileName::new(),
            buf: [0; IO_BUFFER_SIZE],
            buf_pos: 0,
            file_pos: 0,
        }
    }

    pub fn run(&mut self) {
        let action = self.input.read();
        self.execute(action);
    }

    fn execute(&mut self, action: input::Action) {
        match action {
            input::Action::Reset => self.handle_reset(),
            input::Action::Stop => self.handle_stop(),
            input::Action::Data(payload) => self.handle_data(payload),
        }
    }

    fn handle_reset(&mut self) {
        self.buf_pos = 0;
        self.file_pos = 0;
        self.mode = Mode::Ready;
        self.indicators.system_error_off();
        self.indicators.write_off();
        self.indicators.read_off();
        self.output.write(output::Frame::Ack);
    }

    fn handle_stop(&mut self) {
        if self.buf_pos > 0 {
            self.handle_dump_payload();
        }

        self.indicators.write_off();
        self.indicators.read_off();
        self.handle_reset();
    }

    fn handle_data(&mut self, payload: u16) {
        match self.mode {
            Mode::Ready => match input::Frame::from(payload) {
                input::Frame::CheckStatus => self.handle_check_status(),
                input::Frame::Address(address) => self.handle_address(address),
                _ => self.handle_error(AppError::UnhandledReadyCommand),
            },
            Mode::Address => match input::Frame::from(payload) {
                input::Frame::Read => self.handle_read(),
                input::Frame::Write => self.handle_write(),
                _ => self.handle_error(AppError::UnhandledAddressCommand),
            },
            Mode::Read => self.handle_read_payload(),
            Mode::Write => self.handle_write_payload(payload),
            Mode::Error(opcode) => self.handle_error(opcode),
        }
    }

    fn handle_check_status(&mut self) {
        if self.card.is_attached() {
            self.output.write(output::Frame::Ack);
        } else {
            self.handle_error(AppError::SdmmcDetached);
        }
    }

    fn handle_address(&mut self, address: u16) {
        self.mode = Mode::Address;
        self.file_name = sdmmc::FileName::from(address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_read(&mut self) {
        match self.read_buf_from_card(0) {
            Ok(size) => {
                self.file_pos = size;
                self.mode = Mode::Read;
                self.indicators.read_on();
                self.output.write(output::Frame::Ack);
            }
            Err(error) => self.handle_error(error),
        }
    }

    fn handle_write(&mut self) {
        match self.remove_file() {
            Ok(_) => {
                self.mode = Mode::Write;
                self.indicators.write_on();
                self.output.write(output::Frame::Ack);
            }
            Err(error) => self.handle_error(error),
        }
    }

    fn remove_file(&mut self) -> Result<(), AppError> {
        let mut controller = self.card.open()?;
        if controller.is_file_exists(&self.file_name)? {
            controller.delete_file(&self.file_name)?;
        }
        Ok(())
    }

    fn handle_read_payload(&mut self) {
        if self.buf_pos + 1 > self.buf.len() {
            self.buf.iter_mut().for_each(|byte| *byte = 0);
            match self.read_buf_from_card(self.file_pos) {
                Ok(size) => {
                    self.buf_pos = 0;
                    self.file_pos += size;
                    self.handle_send_buf_chunk();
                }
                Err(error) => self.handle_error(error),
            }
        } else {
            self.handle_send_buf_chunk();
        }
    }

    fn handle_send_buf_chunk(&mut self) {
        let payload = u16::from_le_bytes([self.buf[self.buf_pos], self.buf[self.buf_pos + 1]]);
        self.output.write(output::Frame::Data(payload));
        self.buf_pos += 2;
    }

    fn read_buf_from_card(&mut self, offset: usize) -> Result<usize, AppError> {
        let mut controller = self.card.open()?;
        let mut file = controller.open_file_read(&self.file_name)?;
        file.seek_from_start(offset as u32)?;
        controller.read(&mut file, &mut self.buf)
    }

    fn handle_write_payload(&mut self, payload: u16) {
        if self.buf_pos + 1 <= self.buf.len() {
            let bytes = payload.to_le_bytes();
            self.buf[self.buf_pos] = bytes[0];
            self.buf[self.buf_pos + 1] = bytes[1];
            self.buf_pos += 2;
            self.output.write(output::Frame::Ack)
        } else {
            self.handle_dump_payload();
            self.handle_write_payload(payload);
        }
    }

    fn handle_dump_payload(&mut self) {
        match self.write_buf_to_card() {
            Ok(_) => {
                self.buf_pos = 0;
                self.output.write(output::Frame::Ack)
            }
            Err(error) => self.handle_error(error),
        }
    }

    fn write_buf_to_card(&mut self) -> Result<usize, AppError> {
        let mut controller = self.card.open()?;
        let mut file = controller.oped_file_append(&self.file_name)?;
        let size = controller.write(&mut file, &self.buf[0..self.buf_pos])?;
        controller.close_file(file)?;
        controller.close();
        Ok(size)
    }

    fn handle_error<T: Into<u16>>(&mut self, error: T) {
        let opcode = error.into();
        self.mode = Mode::Error(opcode);
        self.indicators.system_error_on();
        self.output.write(output::Frame::Error(opcode));
    }
}
