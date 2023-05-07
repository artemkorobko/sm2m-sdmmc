use crate::peripherals::sm2m;

enum Mode {
    Ready,
}

pub struct Device {
    input: sm2m::input::Bus,
    output: sm2m::output::Bus,
    mode: Mode,
}

impl Device {
    pub fn new(input: sm2m::input::Bus, output: sm2m::output::Bus) -> Self {
        Self {
            input,
            output,
            mode: Mode::Ready,
        }
    }

    pub fn run(&mut self) {
        self.process_input();
        self.output.write(sm2m::output::Frame::Ack);
    }

    pub fn process_input(&mut self) {
        match self.input.read() {
            sm2m::input::Frame::Reset => {
                defmt::println!("Received RESET");
            }
            sm2m::input::Frame::EndOfDataTransfer => {
                defmt::println!("End of data transfer");
            }
            sm2m::input::Frame::Payload(data) => {
                defmt::println!("Received PAYLOAD: {:#018b}", data);
            }
        }
    }
}
