use stm32f1xx_hal::{device::Peripherals, pac};

pub struct InputState {
    pub data: u16,
    pub reset: bool,
    pub ctrl0: bool,
    pub ctrl1: bool,
    pub dtc: bool,
}

impl InputState {
    pub fn invert(self) -> Self {
        Self {
            data: !self.data,
            reset: !self.reset,
            ctrl0: !self.ctrl0,
            ctrl1: !self.ctrl1,
            dtc: !self.dtc,
        }
    }
}

pub struct OutputState {
    pub data: u16,
    pub reset: bool,
    pub ctrl: bool,
    pub ctrl0: bool,
    pub ctrl1: bool,
    pub ext_err: bool,
    pub complete: bool,
    pub ready: bool,
    pub ext_set: bool,
}

impl OutputState {
    pub fn invert(self) -> Self {
        Self {
            data: !self.data,
            reset: !self.reset,
            ctrl: !self.ctrl,
            ctrl0: !self.ctrl0,
            ctrl1: !self.ctrl1,
            ext_err: !self.ext_err,
            complete: !self.complete,
            ready: !self.ready,
            ext_set: !self.ext_set,
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
            bits = write_bit(bits, 8, data, 3); // pa8 - Data_Bus_Output_03
            bits = write_flag(bits, 9, state.ctrl1); // pa9 - Data_Bus_Output_Control_1
            bits = write_bit(bits, 10, data, 2); // pa10 - Data_Bus_Output_02
            bits = write_flag(bits, 11, state.ctrl0); // pa11 - Data_Bus_Output_Control_0
            bits = write_bit(bits, 12, data, 1); // pa12 - Data_Bus_Output_01
            bits = write_flag(bits, 15, state.ext_err); // pa15 - External_Error_Output
            w.bits(bits)
        });

        self.device.GPIOB.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 12, state.reset); // pb12 - External_Reset_Output
            bits = write_bit(bits, 15, data, 14); // pb15 - Data_Bus_Output_14
            w.bits(bits)
        });

        self.device.GPIOC.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_flag(bits, 3, state.ext_set); // pc3 - External_Set_Output
            bits = write_bit(bits, 6, data, 9); // pc6 - Data_Bus_Output_09
            bits = write_bit(bits, 7, data, 6); // pc7 - Data_Bus_Output_06
            bits = write_bit(bits, 8, data, 5); // pc8 - Data_Bus_Output_05
            bits = write_bit(bits, 9, data, 4); // pc9 - Data_Bus_Output_04
            bits = write_bit(bits, 10, data, 0); // pc10 - Data_Bus_Output_00
            bits = write_flag(bits, 11, state.complete); // pc11 - Data_Transfer_Completed_Output
            bits = write_flag(bits, 12, state.ctrl); // pc12 - Data_Bus_Output_Control_State
            w.bits(bits)
        });

        self.device.GPIOD.odr.modify(|r, w| {
            let mut bits = r.bits();
            bits = write_bit(bits, 8, data, 11); // pd8 - Data_Bus_Output_11
            bits = write_bit(bits, 9, data, 15); // pd9 - Data_Bus_Output_15
            bits = write_flag(bits, 10, state.ready); // pd10 - Ready_Output
            bits = write_bit(bits, 11, data, 12); // pd11 - Data_Bus_Output_12
            bits = write_bit(bits, 12, data, 13); // pd12 - Data_Bus_Output_13
            bits = write_bit(bits, 13, data, 10); // pd13 - Data_Bus_Output_10
            bits = write_bit(bits, 14, data, 8); // pd14 - Data_Bus_Output_08
            bits = write_bit(bits, 15, data, 7); // pd15 - Data_Bus_Output_07
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

        let data = copy_bit(pb, 4, 8) // pb4 - Data_Bus_Input_08
        | copy_bit(pb, 7, 0) // pb7 - Data_Bus_Input_00
        | copy_bit(pd, 0, 15) // pd0 - Data_Bus_Input_15
        | copy_bit(pd, 1, 13) // pd1 - Data_Bus_Input_13
        | copy_bit(pd, 3, 14) // pd3 - Data_Bus_Input_14
        | copy_bit(pd, 4, 12) // pd4 - Data_Bus_Input_12
        | copy_bit(pd, 5, 10) // pd5 - Data_Bus_Input_10
        | copy_bit(pd, 6, 9) // pd6 - Data_Bus_Input_09
        | copy_bit(pd, 7, 11) // pd7 - Data_Bus_Input_11
        | copy_bit(pe, 0, 2) // pe0 - Data_Bus_Input_02
        | copy_bit(pe, 1, 1) // pe1 - Data_Bus_Input_01
        | copy_bit(pe, 2, 3) // pe2 - Data_Bus_Input_03
        | copy_bit(pe, 3, 4) // pe3 - Data_Bus_Input_04
        | copy_bit(pe, 4, 6) // pe4 - Data_Bus_Input_06
        | copy_bit(pe, 5, 5) // pe5 - Data_Bus_Input_05
        | copy_bit(pe, 6, 7); // pe6 - Data_Bus_Input_07

        let reset = bit_is_set(pb, 9); // pb9 - Reset_Input
        let ctrl0 = bit_is_set(pd, 2); // pd2 - Data_Bus_Input_Control_0
        let ctrl1 = bit_is_set(pb, 8); // pb8 - Data_Bus_Input_Control_1
        let dtc = bit_is_set(pb, 14); // pb14 - Data_Transfer_Control_Completed_Input

        InputState {
            data,
            reset,
            ctrl0,
            ctrl1,
            dtc,
        }
    }
}
