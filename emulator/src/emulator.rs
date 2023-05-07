use stm32f1xx_hal::gpio;

use crate::{input, output};

macro_rules! log {
    ($enabled:expr, $($arg:tt)+) => {
        if $enabled {
            defmt::println!($($arg)+);
        }
    };
}

enum Mode {
    Read,
    Write,
}

enum State {
    Ready,
    ResetSent,
    CheckStatusSent,
    AddressSent,
    ReadSent(u16),
    WriteSent(u16),
    StopSent,
}

pub type LedPin = gpio::Pin<'D', 7, gpio::Output>;

pub struct Machine {
    input: input::Bus,
    output: output::Bus,
    led: LedPin,
    state: State,
    mode: Mode,
    debug: bool,
    last_address: u16,
}

impl Machine {
    pub fn new(input: input::Bus, output: output::Bus, led: LedPin) -> Self {
        Self {
            input,
            output,
            led,
            state: State::Ready,
            mode: Mode::Write,
            debug: false,
            last_address: 0,
        }
    }

    pub fn start_write(&mut self, debug: bool) {
        self.mode = Mode::Write;
        self.debug = debug;
        self.start();
    }

    pub fn start_read(&mut self, debug: bool) {
        self.mode = Mode::Read;
        self.debug = debug;
        self.start();
    }

    pub fn step(&mut self) {
        self.process_state();
    }

    pub fn stop(&mut self) {
        log!(self.debug, "Send STOP");
        self.state = State::StopSent;
        self.output.write_reversed(output::Frame::Stop);
    }

    fn start(&mut self) {
        self.led.set_high();
        self.state = State::Ready;
        self.step();
    }

    fn process_state(&mut self) {
        match self.state {
            State::Ready => {
                log!(self.debug, "Send RESET: {}", self.debug);
                self.state = State::ResetSent;
                self.output.write_reversed(output::Frame::Reset);
            }
            State::ResetSent => {
                if let Some(_) = self.read() {
                    log!(self.debug, "Received READY, send CHECK_STATUS");
                    self.state = State::CheckStatusSent;
                    self.output.write_reversed(output::Frame::CheckStatus);
                }
            }
            State::CheckStatusSent => {
                if let Some(_) = self.read() {
                    log!(self.debug, "Received READY, send ADDRESS");
                    self.state = State::AddressSent;
                    self.last_address += 1;
                    self.output
                        .write_reversed(output::Frame::Address(self.last_address));
                }
            }
            State::AddressSent => {
                if let Some(_) = self.read() {
                    match self.mode {
                        Mode::Read => {
                            log!(self.debug, "Received READY, send READ");
                            self.state = State::ReadSent(0);
                            self.output.write_reversed(output::Frame::Read);
                        }
                        Mode::Write => {
                            let data = 0;
                            log!(self.debug, "Received READY, send WRITE: {}", data);
                            self.state = State::WriteSent(data);
                            self.output.write_reversed(output::Frame::Write(data));
                        }
                    }
                }
            }
            State::ReadSent(count) => {
                if let Some(data) = self.read() {
                    log!(self.debug, "Received READY: {}, send READ", data);
                    self.state = State::ReadSent(count + 1);
                    self.output.write_reversed(output::Frame::Read);
                }
            }
            State::WriteSent(mut data) => {
                if let Some(_) = self.read() {
                    if data < u16::MAX {
                        data += 1;
                        log!(self.debug, "Received READY, send WRITE: {}", data);
                        self.state = State::WriteSent(data);
                        self.output.write_reversed(output::Frame::Write(data));
                    } else {
                        log!(self.debug, "Received READY, send STOP");
                        self.state = State::StopSent;
                        self.output.write_reversed(output::Frame::Stop);
                    }
                }
            }
            State::StopSent => {
                self.read();
                log!(self.debug, "Emulation completed");
            }
        }
    }

    fn read(&mut self) -> Option<u16> {
        match self.input.read_reversed() {
            input::Frame::Ack(payload) => {
                log!(self.debug, "Received ACK: {}", payload);
                Some(payload)
            }
            input::Frame::Error(opcode) => {
                log!(self.debug, "Received ERROR: {}", opcode);
                self.led.set_low();
                None
            }
            _ => {
                log!(self.debug, "Received invalid frame");
                self.led.set_low();
                None
            }
        }
    }
}
