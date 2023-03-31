use stm32f1xx_hal::{device::Peripherals, pac};

pub struct InputState {
    pub data: u16,
    pub rst: bool,
    pub ctrli_0: bool,
    pub ctrli_1: bool,
    pub dte: bool,
}

impl InputState {
    pub fn invert(self) -> Self {
        Self {
            data: !self.data,
            rst: !self.rst,
            ctrli_0: !self.ctrli_0,
            ctrli_1: !self.ctrli_1,
            dte: !self.dte,
        }
    }
}

pub struct OutputState {
    pub data: u16,
    pub rste: bool,
    pub ctrld: bool,
    pub ctrlo_0: bool,
    pub ctrlo_1: bool,
    pub err: bool,
    pub dteo: bool,
    pub rdy: bool,
    pub sete: bool,
}

impl OutputState {
    pub fn invert(self) -> Self {
        Self {
            data: !self.data,
            rste: !self.rste,
            ctrld: !self.ctrld,
            ctrlo_0: !self.ctrlo_0,
            ctrlo_1: !self.ctrlo_1,
            err: !self.err,
            dteo: !self.dteo,
            rdy: !self.rdy,
            sete: !self.sete,
        }
    }
}

pub struct Bus {
    device: Peripherals,
}

impl Bus {
    fn take() -> Option<Self> {
        let device = pac::Peripherals::take()?;
        Some(Self { device })
    }

    unsafe fn write(&self, state: &OutputState) {
        fn write_bit(to: u32, dst: u32, from: u32, src: u32) -> u32 {
            let bit = (from >> src) & 1;
            let clean = to & !(1 << dst);
            clean | (bit << dst)
        }

        fn write_flag(to: u32, dst: u32, flag: bool) -> u32 {
            let clean = to & !(1 << dst);
            clean | flag as u32
        }

        let data = state.data as u32;

        self.device.GPIOA.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_bit(bits, 8, data, 3); // pa8 - DO_3
            bits = write_flag(bits, 9, state.ctrlo_1); // pa9 - CTRLO_1
            bits = write_bit(bits, 10, data, 2); // pa10 - DO_2
            bits = write_flag(bits, 11, state.ctrlo_0); // pa11 - CTRLO_0
            bits = write_bit(bits, 12, data, 1); // pa12 - DO_1
            bits = write_flag(bits, 15, state.err); // pa15 - ERR
            w.bits(bits)
        });

        self.device.GPIOB.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 12, state.rste); // pb12 - RSTE
            bits = write_bit(bits, 15, data, 14); // pb15 - DO_14
            w.bits(bits)
        });

        self.device.GPIOC.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 3, state.sete); // pc3 - SETE
            bits = write_bit(bits, 6, data, 9); // pc6 - DO_9
            bits = write_bit(bits, 7, data, 6); // pc7 - DO_6
            bits = write_bit(bits, 8, data, 5); // pc8 - DO_5
            bits = write_bit(bits, 9, data, 4); // pc9 - DO_4
            bits = write_bit(bits, 10, data, 0); // pc10 - DO_0
            bits = write_flag(bits, 11, state.dteo); // pc11 - DTEO
            bits = write_flag(bits, 12, state.ctrld); // pc12 - CTRLD
            w.bits(bits)
        });

        self.device.GPIOD.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_bit(bits, 8, data, 11); // pd8 - DO_11
            bits = write_bit(bits, 9, data, 15); // pd9 - DO_15
            bits = write_flag(bits, 10, state.rdy); // pd10 - RDY
            bits = write_bit(bits, 11, data, 12); // pd11 - DO_12
            bits = write_bit(bits, 12, data, 13); // pd12 - DO_13
            bits = write_bit(bits, 13, data, 10); // pd13 - DO_10
            bits = write_bit(bits, 14, data, 8); // pd14 - DO_8
            bits = write_bit(bits, 15, data, 7); // pd15 - DO_7
            w.bits(bits)
        });
    }

    unsafe fn read(&self) -> InputState {
        fn copy_bit(from: u16, src: u16, dst: u16) -> u16 {
            let masked = from & (1 << src);
            if src > dst {
                masked >> (src - dst)
            } else {
                masked << (dst - src)
            }
        }

        fn bit_is_set(from: u16, bit: u16) -> bool {
            (from >> bit) & 1 == 1
        }

        let pb = self.device.GPIOB.idr.read().bits() as u16;
        let pd = self.device.GPIOD.idr.read().bits() as u16;
        let pe = self.device.GPIOE.idr.read().bits() as u16;

        let data = copy_bit(pb, 4, 8) // pb4 - DI_8
        | copy_bit(pb, 7, 0) // pb7 - DI_0
        | copy_bit(pd, 0, 15) // pd0 - DI_15
        | copy_bit(pd, 1, 13) // pd1 - DI_13
        | copy_bit(pd, 3, 14) // pd3 - DI_14
        | copy_bit(pd, 4, 12) // pd4 - DI_12
        | copy_bit(pd, 5, 10) // pd5 - DI_10
        | copy_bit(pd, 6, 9) // pd6 - DI_9
        | copy_bit(pd, 7, 11) // pd7 - DI_11
        | copy_bit(pe, 0, 2) // pe0 - DI_2
        | copy_bit(pe, 1, 1) // pe1 - DI_1
        | copy_bit(pe, 2, 3) // pe2 - DI_3
        | copy_bit(pe, 3, 4) // pe3 - DI_4
        | copy_bit(pe, 4, 6) // pe4 - DI_6
        | copy_bit(pe, 5, 5) // pe5 - DI_5
        | copy_bit(pe, 6, 7); // pe6 - DI_7

        let reset = bit_is_set(pb, 9); // pb9 - RST
        let ctrl0 = bit_is_set(pd, 2); // pd2 - CTRLI_0
        let ctrl1 = bit_is_set(pb, 8); // pb8 - CTRLI_1
        let dtc = bit_is_set(pb, 14); // pb14 - DTEI

        InputState {
            data,
            rst: reset,
            ctrli_0: ctrl0,
            ctrli_1: ctrl1,
            dte: dtc,
        }
    }
}
