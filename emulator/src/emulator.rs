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
    pub fn new(input: input::Bus, output: output::Bus, led: LedPin, debug: bool) -> Self {
        Self {
            input,
            output,
            led,
            state: State::Ready,
            mode: Mode::Write,
            debug,
            last_address: 0,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn start_write(&mut self, debug: bool) {
        defmt::println!("Start write emulation with debug: {}", debug);
        self.mode = Mode::Write;
        self.debug = debug;
        self.start();
    }

    pub fn start_read(&mut self, debug: bool) {
        defmt::println!("Start read emulation with debug: {}", debug);
        self.mode = Mode::Read;
        self.debug = debug;
        self.start();
    }

    pub fn step(&mut self) {
        match self.state {
            State::Ready => {
                log!(self.debug, "Send reset");
                self.state = State::Reset;
                self.output.write(output::Frame::Reset);
            }
            State::Reset => {
                if self.read().is_some() {
                    log!(self.debug, "Send check status");
                    self.state = State::CheckStatus;
                    self.output.write(output::Frame::CheckStatus);
                }
            }
            State::CheckStatus => {
                if self.read().is_some() {
                    self.last_address += 1;
                    log!(self.debug, "Send address: {}", self.last_address);
                    self.state = State::Address;
                    self.output.write(output::Frame::Address(self.last_address));
                }
            }
            State::Address => {
                if self.read().is_some() {
                    match self.mode {
                        Mode::Read => {
                            log!(self.debug, "Send read");
                            self.state = State::Read(0);
                            self.output.write(output::Frame::Read);
                        }
                        Mode::Write => {
                            log!(self.debug, "Send write");
                            self.state = State::Write(0);
                            self.output.write(output::Frame::Write);
                        }
                    }
                }
            }
            State::Read(mut count) => {
                if let Some(data) = self.read() {
                    if count < 10 {
                        count += 1;

                        if count == 1 {
                            log!(self.debug, "Send read data {}", count);
                        } else {
                            log!(self.debug, "Read: {}, send read data {}", data, count);
                        }

                        self.state = State::Read(count);
                        self.output.write(output::Frame::ReadData);
                    } else {
                        log!(self.debug, "Read: {}, send stop", data);
                        self.state = State::Stop;
                        self.output.write(output::Frame::Stop);
                    }
                }
            }
            State::Write(mut data) => {
                if self.read().is_some() {
                    if data < 10 {
                        data += 1;
                        log!(self.debug, "Send write data: {}", data);
                        self.state = State::Write(data);
                        self.output.write(output::Frame::WriteData(data));
                    } else {
                        log!(self.debug, "Send stop");
                        self.state = State::Stop;
                        self.output.write(output::Frame::Stop);
                    }
                }
            }
            State::Stop => {
                if self.read().is_some() {
                    match self.mode {
                        Mode::Write => defmt::println!("Write emulation completed"),
                        Mode::Read => defmt::println!("Read emulation completed"),
                    }
                }
            }
        }
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

    fn read(&mut self) -> Option<u16> {
        match self.input.read() {
            input::Frame::Data(payload) => Some(payload),
            input::Frame::Error(opcode) => {
                log!(self.debug, "Received error: {}", opcode);
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
