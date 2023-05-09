use crate::{
    error::AppError,
    peripherals::sm2m::{input, output},
};

enum Mode {
    Ready,
    Address(u16),
    Read(u16),
    Write(u16),
}

pub struct Device {
    input: input::Bus,
    output: output::Bus,
    mode: Mode,
}

impl Device {
    pub fn new(input: input::Bus, output: output::Bus) -> Self {
        Self {
            input,
            output,
            mode: Mode::Ready,
        }
    }

    pub fn run(&mut self) {
        let action = self.input.read();
        self.process_action(action);
        self.output.write(output::Frame::Ack);
    }

    pub fn process_action(&mut self, action: input::Action) {
        match action {
            input::Action::Reset => self.handle_reset(),
            input::Action::End => self.handle_end(),
            input::Action::Data(payload) => self.handle_data(payload),
        }
    }

    fn handle_reset(&mut self) {
        defmt::println!("Handle reset");
        self.mode = Mode::Ready;
    }

    fn handle_end(&mut self) {
        defmt::println!("Handle end");
        self.handle_reset();
    }

    fn handle_data(&mut self, payload: u16) {
        match self.mode {
            Mode::Ready => match input::Frame::from(payload) {
                input::Frame::CheckStatus => self.handle_check_status(),
                input::Frame::Address(address) => self.handle_address(address),
                _ => {
                    defmt::println!("Received unhandled frame in READY state: {}", payload);
                    self.handle_error()
                }
            },
            Mode::Address(address) => match input::Frame::from(payload) {
                input::Frame::Read => self.handle_read(address),
                input::Frame::Write => self.handle_write(address),
                _ => {
                    defmt::println!("Received unhandled frame in ADDRESS state: {}", payload);
                    self.handle_error()
                }
            },
            Mode::Read(address) => self.handle_read_payload(address),
            Mode::Write(address) => self.handle_write_payload(address, payload),
        }
    }

    fn handle_check_status(&mut self) {
        defmt::println!("Handle check status");
        self.output.write(output::Frame::Ack);
    }

    fn handle_address(&mut self, address: u16) {
        defmt::println!("Handle address: {}", address);
        self.mode = Mode::Address(address);
    }

    fn handle_read(&mut self, address: u16) {
        defmt::println!("Handle read");
        self.mode = Mode::Read(address);
    }

    fn handle_write(&mut self, address: u16) {
        defmt::println!("Handle write");
        self.mode = Mode::Write(address);
    }

    fn handle_read_payload(&mut self, address: u16) {
        defmt::println!("Handle read payload from address {}", address);
    }

    fn handle_write_payload(&mut self, address: u16, payload: u16) {
        defmt::println!("Handle write payload {} to address {}", payload, address);
    }

    fn handle_error(&mut self) {
        let opcode = AppError::UnhandledCommand.opcode();
        self.output.write(output::Frame::Error(opcode))
    }
}
