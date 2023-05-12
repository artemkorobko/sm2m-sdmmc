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

#[derive(Eq, PartialEq)]
enum State {
    Ready,
    Reset,
    CheckStatus,
    Address,
    Read,
    ReadData(usize),
    Write,
    WriteData(usize),
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
    transfers: usize,
    last_received: u16,
    last_address: u16,
    last_data: u16,
}

impl Machine {
    pub fn new(
        input: input::Bus,
        output: output::Bus,
        led: LedPin,
        transfers: usize,
        debug: bool,
    ) -> Self {
        Self {
            input,
            output,
            led,
            state: State::Ready,
            mode: Mode::Write,
            debug,
            transfers,
            last_received: 0,
            last_address: 0,
            last_data: 0,
        }
    }

    pub fn is_completed(&self) -> bool {
        self.state == State::Stop
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn set_transfers(&mut self, transfers: usize) {
        self.transfers = transfers;
    }

    pub fn start_write(&mut self, debug: bool) {
        defmt::println!(
            "Start {} bytes write simulation with debug: {}",
            self.transfers * 2,
            debug
        );
        self.mode = Mode::Write;
        self.debug = debug;
        self.start();
    }

    pub fn start_read(&mut self, debug: bool) {
        defmt::println!(
            "Start {} bytes read simulation with debug: {}",
            self.transfers * 2,
            debug
        );
        self.mode = Mode::Read;
        self.debug = debug;
        self.start();
    }

    pub fn step(&mut self) {
        match self.state {
            State::Ready => {
                self.state = State::Reset;
                self.output.write(output::Frame::Reset);
            }
            State::Reset => {
                if self.read().is_some() {
                    self.state = State::CheckStatus;
                    self.output.write(output::Frame::CheckStatus);
                }
            }
            State::CheckStatus => {
                if self.read().is_some() {
                    self.last_address += 1;
                    self.state = State::Address;
                    self.output.write(output::Frame::Address(self.last_address));
                }
            }
            State::Address => {
                if self.read().is_some() {
                    match self.mode {
                        Mode::Read => {
                            self.state = State::Read;
                            self.output.write(output::Frame::Read);
                        }
                        Mode::Write => {
                            self.state = State::Write;
                            self.output.write(output::Frame::Write);
                        }
                    }
                }
            }
            State::Read => {
                if self.read().is_some() {
                    self.state = State::ReadData(1);
                    self.output.write(output::Frame::ReadData);
                }
            }
            State::ReadData(mut count) => {
                if let Some(data) = self.read() {
                    if data == count as u16 - 1 {
                        self.last_received = data;
                        if count < self.transfers {
                            count += 1;
                            self.state = State::ReadData(count);
                            self.output.write(output::Frame::ReadData);
                        } else {
                            self.state = State::Stop;
                            self.output.write(output::Frame::Stop);
                        }
                    } else {
                        defmt::println!(
                            "Invalid data read, expected: {}, received: {}, last received {}",
                            count,
                            data,
                            self.last_received,
                        );
                    }
                }
            }
            State::Write => {
                if self.read().is_some() {
                    self.state = State::WriteData(1);
                    self.output.write(output::Frame::WriteData(0));
                }
            }
            State::WriteData(count) => {
                if self.read().is_some() {
                    if count < self.transfers {
                        self.state = State::WriteData(count + 1);
                        self.output.write(output::Frame::WriteData(count as u16));
                    } else {
                        defmt::println!("Done, last sent data {}", count - 1);
                        self.state = State::Stop;
                        self.output.write(output::Frame::Stop);
                    }
                }
            }
            State::Stop => {
                if self.read().is_some() {
                    match self.mode {
                        Mode::Write => defmt::println!("Write simulation completed"),
                        Mode::Read => defmt::println!(
                            "Read simulation completed, last received {}",
                            self.last_received
                        ),
                    }
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.state = State::Stop;
        self.last_address = 0;
        self.last_received = 0;
        self.output.write(output::Frame::Stop);
    }

    pub fn read(&mut self) -> Option<u16> {
        match self.input.read() {
            input::Frame::Data(payload) => Some(payload),
            _ => {
                log!(self.debug, "Received invalid frame");
                self.led.set_low();
                None
            }
        }
    }

    fn start(&mut self) {
        self.led.set_high();
        self.last_data = 0;
        self.state = State::Ready;
        self.step();
    }
}
