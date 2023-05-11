use heapless::Vec;

use crate::{
    error::AppError,
    peripherals::{
        sdmmc,
        sm2m::{input, output},
    },
};

enum Mode {
    Ready,
    Address(u16),
    Read(u16),
    Write(u16),
    Error(u16),
}

const IO_BUFFER_SIZE: usize = 1024;

pub struct Device {
    input: input::Bus,
    output: output::Bus,
    card: sdmmc::Card,
    mode: Mode,
    buf: Vec<u8, IO_BUFFER_SIZE>,
}

impl Device {
    pub fn new(input: input::Bus, output: output::Bus, card: sdmmc::Card) -> Self {
        Self {
            input,
            output,
            card,
            mode: Mode::Ready,
            buf: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let action = self.input.read();
        self.execute_action(action);
    }

    fn execute_action(&mut self, action: input::Action) {
        match action {
            input::Action::Reset => self.handle_reset(),
            input::Action::End => self.handle_end(),
            input::Action::Data(payload) => self.handle_data(payload),
        }
    }

    fn handle_reset(&mut self) {
        defmt::println!("Handle reset");
        self.mode = Mode::Ready;
        self.output.write(output::Frame::Ack);
    }

    fn handle_end(&mut self) {
        defmt::println!("Handle end");
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
            Mode::Address(address) => match input::Frame::from(payload) {
                input::Frame::Read => self.handle_read(address),
                input::Frame::Write => self.handle_write(address),
                _ => self.handle_error(AppError::UnhandledAddressCommand),
            },
            Mode::Read(address) => self.handle_read_payload(address),
            Mode::Write(address) => self.handle_write_payload(address, payload),
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
        self.mode = Mode::Address(address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_read(&mut self, address: u16) {
        defmt::println!("Handle read");
        self.mode = Mode::Read(address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_write(&mut self, address: u16) {
        defmt::println!("Handle write");
        self.mode = Mode::Write(address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_read_payload(&mut self, address: u16) {
        defmt::println!("Handle read payload from address {}", address);
        self.output.write(output::Frame::Data(0));
    }

    fn handle_write_payload(&mut self, address: u16, payload: u16) {
        defmt::println!("Handle write payload {} to address {}", payload, address);
        self.output.write(output::Frame::Ack);
    }

    fn handle_error<T: Into<u16>>(&mut self, error: T) {
        let opcode = error.into();
        defmt::println!("Write error: {}", opcode);
        self.mode = Mode::Error(opcode);
        self.output.write(output::Frame::Error(opcode));
    }
}
