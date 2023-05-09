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
    Write,
    Read,
}

enum State {
    Ready,
    Reset,
    CheckStatus,
    Address,
    Read(u16),
    Write(u16),
    Stop,
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
        self.state = State::Stop;
        self.output.write(output::Frame::Stop);
    }

    fn start(&mut self) {
        self.led.set_high();
        self.state = State::Ready;
        self.step();
    }

    fn process_state(&mut self) {
        match self.state {
            State::Ready => {
                log!(self.debug, "Send RESET");
                self.state = State::Reset;
                self.output.write(output::Frame::Reset);
            }
            State::Reset => {
                if let Some(_) = self.read() {
                    log!(self.debug, "Send CHECK_STATUS");
                    self.state = State::CheckStatus;
                    self.output.write(output::Frame::CheckStatus);
                }
            }
            State::CheckStatus => {
                if self.read().is_some() {
                    self.last_address += 1;
                    log!(self.debug, "Send ADDRESS: {}", self.last_address);
                    self.state = State::Address;
                    self.output.write(output::Frame::Address(self.last_address));
                }
            }
            State::Address => {
                if self.read().is_some() {
                    match self.mode {
                        Mode::Read => {
                            log!(self.debug, "Send READ");
                            self.state = State::Read(0);
                            self.output.write(output::Frame::Read);
                        }
                        Mode::Write => {
                            log!(self.debug, "Send WRITE");
                            self.state = State::Write(0);
                            self.output.write(output::Frame::Write);
                        }
                    }
                }
            }
            State::Read(count) => {
                if let Some(data) = self.read() {
                    if data < 10 {
                        // First read will receive ACK. Every following read will receive DATA.
                        log!(self.debug, "Read: {}, send READ_DATA", data);
                        self.state = State::Read(count + 1);
                        self.output.write(output::Frame::ReadData);
                    } else {
                        log!(self.debug, "Read: {}, send STOP", data);
                        self.state = State::Stop;
                        self.output.write(output::Frame::Stop);
                    }
                }
            }
            State::Write(mut data) => {
                if self.read().is_some() {
                    if data < 10 {
                        data += 1;
                        log!(self.debug, "Send WRITE_DATA: {}", data);
                        self.state = State::Write(data);
                        self.output.write(output::Frame::WriteData(data));
                    } else {
                        log!(self.debug, "Send STOP");
                        self.state = State::Stop;
                        self.output.write(output::Frame::Stop);
                    }
                }
            }
            State::Stop => {
                if self.read().is_some() {
                    log!(self.debug, "Emulation completed");
                }
            }
        }
    }

    fn read(&mut self) -> Option<u16> {
        match self.input.read() {
            input::Frame::Data(payload) => Some(payload),
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
