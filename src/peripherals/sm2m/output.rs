use stm32f1xx_hal::device::{Peripherals, GPIOA, GPIOB, GPIOC, GPIOD};

#[derive(Default)]
pub struct Output {
    data: u16,
    ctrlo_0: bool,
    ctrlo_1: bool,
    rste: bool,
    ctrld: bool,
    err: bool,
    dteo: bool,
    rdy: bool,
    sete: bool,
}

impl Output {
    pub fn ok() -> Self {
        let mut output = Self::default();
        output.rdy = true;
        output
    }

    pub fn error() -> Self {
        let mut output = Self::default();
        output.err = true;
        output
    }

    pub fn data(data: u16) -> Self {
        let mut output = Self::default();
        output.data = data;
        output.rdy = true;
        output
    }
}

pub trait Write {
    unsafe fn write(&mut self, output: &Output);
}

pub struct OutputBus {
    gpioa: GPIOA,
    gpiob: GPIOB,
    gpioc: GPIOC,
    gpiod: GPIOD,
}

impl OutputBus {
    pub unsafe fn steal() -> Self {
        let peripherals = Peripherals::steal();

        Self {
            gpioa: peripherals.GPIOA,
            gpiob: peripherals.GPIOB,
            gpioc: peripherals.GPIOC,
            gpiod: peripherals.GPIOD,
        }
    }
}

impl Write for OutputBus {
    unsafe fn write(&mut self, output: &Output) {
        fn write_bit(to: u32, dst: u32, from: u32, src: u32) -> u32 {
            let bit = (from >> src) & 1;
            let clean = to & !(1 << dst);
            clean | (bit << dst)
        }

        fn write_flag(to: u32, dst: u32, flag: bool) -> u32 {
            let clean = to & !(1 << dst);
            clean | flag as u32
        }

        let data = output.data as u32;

        self.gpioa.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_bit(bits, 8, data, 3); // pa8 - DO_3
            bits = write_flag(bits, 9, output.ctrlo_1); // pa9 - CTRLO_1
            bits = write_bit(bits, 10, data, 2); // pa10 - DO_2
            bits = write_flag(bits, 11, output.ctrlo_0); // pa11 - CTRLO_0
            bits = write_bit(bits, 12, data, 1); // pa12 - DO_1
            bits = write_flag(bits, 15, output.err); // pa15 - ERR
            w.bits(bits)
        });

        self.gpiob.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 12, output.rste); // pb12 - RSTE
            bits = write_bit(bits, 15, data, 14); // pb15 - DO_14
            w.bits(bits)
        });

        self.gpioc.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 3, output.sete); // pc3 - SETE
            bits = write_bit(bits, 6, data, 9); // pc6 - DO_9
            bits = write_bit(bits, 7, data, 6); // pc7 - DO_6
            bits = write_bit(bits, 8, data, 5); // pc8 - DO_5
            bits = write_bit(bits, 9, data, 4); // pc9 - DO_4
            bits = write_bit(bits, 10, data, 0); // pc10 - DO_0
            bits = write_flag(bits, 11, output.dteo); // pc11 - DTEO
            bits = write_flag(bits, 12, output.ctrld); // pc12 - CTRLD
            w.bits(bits)
        });

        self.gpiod.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_bit(bits, 8, data, 11); // pd8 - DO_11
            bits = write_bit(bits, 9, data, 15); // pd9 - DO_15
            bits = write_bit(bits, 11, data, 12); // pd11 - DO_12
            bits = write_bit(bits, 12, data, 13); // pd12 - DO_13
            bits = write_bit(bits, 13, data, 10); // pd13 - DO_10
            bits = write_bit(bits, 14, data, 8); // pd14 - DO_8
            bits = write_bit(bits, 15, data, 7); // pd15 - DO_7
            w.bits(bits)
        });

        // Notify SM2M with a delay after setting data to all lines.
        // At the time of writing RDY to the bus all lines should alreday be
        // set to corresponding states.
        self.gpiod.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 10, output.rdy); // pd10 - RDY
            w.bits(bits)
        });
    }
}

pub struct InvertedOutputBus(OutputBus);

impl From<OutputBus> for InvertedOutputBus {
    fn from(value: OutputBus) -> Self {
        Self(value)
    }
}

impl Write for InvertedOutputBus {
    unsafe fn write(&mut self, output: &Output) {
        let inverted = Output {
            data: !output.data,
            rste: !output.rste,
            ctrld: !output.ctrld,
            ctrlo_0: !output.ctrlo_0,
            ctrlo_1: !output.ctrlo_1,
            err: !output.err,
            dteo: !output.dteo,
            rdy: !output.rdy,
            sete: !output.sete,
        };

        self.0.write(&inverted)
    }
}
