use stm32f1xx_hal::device::{Peripherals, GPIOB, GPIOD, GPIOE};

pub struct Input {
    pub data: u16,
    pub rst: bool,
    pub ctrli_0: bool,
    pub ctrli_1: bool,
    pub dtei: bool,
}

pub enum Command {
    CheckStatus,
    Reset,
    Write,
    Read,
    Address(u16),
}

impl Command {
    pub fn from(value: Input) -> Option<Self> {
        if value.data & 0x0F == 0x00 {
            Some(Self::CheckStatus)
        } else if value.data & 0x0F == 0x01 {
            Some(Self::Write)
        } else if value.data & 0x0F == 0x02 {
            Some(Self::Read)
        } else if value.data & 0x0F == 0x03 {
            Some(Self::Address(0xFC00))
        } else if value.rst {
            Some(Self::Reset)
        } else {
            None
        }
    }
}

pub trait Read {
    unsafe fn read(&self) -> Input;
}

pub struct InputBus {
    gpiob: GPIOB,
    gpiod: GPIOD,
    gpioe: GPIOE,
}

impl InputBus {
    pub unsafe fn steal() -> Self {
        let peripherals = Peripherals::steal();

        Self {
            gpiob: peripherals.GPIOB,
            gpiod: peripherals.GPIOD,
            gpioe: peripherals.GPIOE,
        }
    }

    fn read_gpiob(&self) -> u16 {
        self.gpiob.idr.read().bits() as u16
    }

    fn read_gpiod(&self) -> u16 {
        self.gpiod.idr.read().bits() as u16
    }

    fn read_gpioe(&self) -> u16 {
        self.gpioe.idr.read().bits() as u16
    }
}

impl Read for InputBus {
    unsafe fn read(&self) -> Input {
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

        let pb = self.read_gpiob();
        let pd = self.read_gpiod();
        let pe = self.read_gpioe();

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

        let rst = bit_is_set(pb, 9); // pb9 - RST
        let ctrli_0 = bit_is_set(pd, 2); // pd2 - CTRLI_0
        let ctrli_1 = bit_is_set(pb, 8); // pb8 - CTRLI_1
        let dtei = bit_is_set(pb, 14); // pb14 - DTEI

        Input {
            data,
            rst,
            ctrli_0,
            ctrli_1,
            dtei,
        }
    }
}

pub struct InvertedInputBus(InputBus);

impl From<InputBus> for InvertedInputBus {
    fn from(value: InputBus) -> Self {
        Self(value)
    }
}

impl Read for InvertedInputBus {
    unsafe fn read(&self) -> Input {
        let input = self.0.read();

        Input {
            data: !input.data,
            rst: !input.rst,
            ctrli_0: !input.ctrli_0,
            ctrli_1: !input.ctrli_1,
            dtei: !input.dtei,
        }
    }
}
