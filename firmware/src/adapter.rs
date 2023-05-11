use crate::{
    error::AppError,
    peripherals::{
        sdmmc,
        sm2m::{input, output},
    },
};

enum Mode {
    Ready,
    Address,
    Read,
    Write,
    Error(u16),
}

const IO_BUFFER_SIZE: usize = 1024;

pub struct Device {
    input: input::Bus,
    output: output::Bus,
    card: sdmmc::Card,
    mode: Mode,
    file_name: sdmmc::FileName,
    buf: [u8; IO_BUFFER_SIZE],
    buf_pos: usize,
}

impl Device {
    pub fn new(input: input::Bus, output: output::Bus, card: sdmmc::Card) -> Self {
        Self {
            input,
            output,
            card,
            mode: Mode::Ready,
            file_name: sdmmc::FileName::new(),
            buf: [0; IO_BUFFER_SIZE],
            buf_pos: 0,
        }
    }

    pub fn run(&mut self) {
        let action = self.input.read();
        self.execute(action);
    }

    fn execute(&mut self, action: input::Action) {
        match action {
            input::Action::Reset => self.handle_reset(),
            input::Action::End => self.handle_end(),
            input::Action::Data(payload) => self.handle_data(payload),
        }
    }

    fn handle_reset(&mut self) {
        defmt::println!("Handle reset");
        self.buf_pos = 0;
        self.mode = Mode::Ready;
        self.output.write(output::Frame::Ack);
    }

    fn handle_end(&mut self) {
        defmt::println!("Handle end");
        if self.buf_pos > 0 {
            self.handle_dump_payload();
        }

        self.handle_reset();
        self.output.write(output::Frame::Ack);
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
            defmt::println!("Error: SDMMC detached");
            self.handle_error(AppError::SdmmcDetached);
        }
    }

    fn handle_address(&mut self, address: u16) {
        defmt::println!("Handle address: {}", address);
        self.mode = Mode::Address;
        self.file_name = sdmmc::FileName::from(address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_read(&mut self) {
        defmt::println!("Handle read");
        self.mode = Mode::Read;
        self.output.write(output::Frame::Ack);
    }

    fn handle_write(&mut self) {
        defmt::println!("Handle write");
        // backup file
        // remove file
        self.mode = Mode::Write;
        self.output.write(output::Frame::Ack);
    }

    fn handle_read_payload(&mut self) {
        defmt::println!("Handle read payload from {}", self.file_name.as_str());
        self.output.write(output::Frame::Data(0));
    }

    fn handle_write_payload(&mut self, payload: u16) {
        if self.buf_pos + 2 <= self.buf.len() {
            defmt::println!("Cache payload {} of {}", self.buf_pos, self.buf.len());
            let bytes = payload.to_le_bytes();
            self.buf[self.buf_pos + 1] = bytes[0];
            self.buf[self.buf_pos + 2] = bytes[1];
            self.buf_pos += 2;
            self.output.write(output::Frame::Ack)
        } else {
            self.handle_dump_payload();
        }
    }

    fn handle_dump_payload(&mut self) {
        defmt::println!("Dump buf to {}", &self.file_name.as_str());
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
        controller.write(&mut file, &self.buf[0..self.buf_pos])
    }

    fn handle_error<T: Into<u16>>(&mut self, error: T) {
        let opcode = error.into();
        defmt::println!("Write error: {}", opcode);
        self.mode = Mode::Error(opcode);
        self.output.write(output::Frame::Error(opcode));
    }
}
