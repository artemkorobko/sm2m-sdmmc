use stm32f1xx_hal::{device, gpio};

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub enum Action {
    Reset,
    Stop,
    Data(u16),
}

pub enum Frame {
    CheckStatus,
    Address(u16),
    Write,
    Read,
    Data(u16),
}

impl Frame {
    pub fn from(payload: u16) -> Self {
        if payload == 0x0000 {
            Self::CheckStatus
        } else if payload == 0x0001 {
            Self::Write
        } else if payload == 0x0002 {
            Self::Read
        } else if payload & 0x0003 == 0x0003 {
            // Bits 10..15 contains the actual address
            Self::Address(payload >> 10)
        } else {
            Self::Data(payload)
        }
    }
}

pub struct Pins {
    pub di_0: Pin<'B', 7>,
    pub di_1: Pin<'E', 1>,
    pub di_2: Pin<'E', 0>,
    pub di_3: Pin<'E', 2>,
    pub di_4: Pin<'E', 3>,
    pub di_5: Pin<'E', 5>,
    pub di_6: Pin<'E', 4>,
    pub di_7: Pin<'E', 6>,
    pub di_8: Pin<'B', 4>,
    pub di_9: Pin<'D', 6>,
    pub di_10: Pin<'D', 5>,
    pub di_11: Pin<'D', 7>,
    pub di_12: Pin<'D', 4>,
    pub di_13: Pin<'D', 1>,
    pub di_14: Pin<'D', 3>,
    pub di_15: Pin<'D', 0>,
    pub ctrli_0: Pin<'D', 2>,
    pub ctrli_1: Pin<'B', 8>,
    pub rsti: Pin<'B', 9>,
    pub dtsi: Pin<'B', 6>, // ignored in favour of DTLI
    // pub dtli: Pin<'B', 13>, // configured outside of the bus
    pub dtei: Pin<'B', 14>,
}

pub struct Bus {
    _pins: Pins,
    gpiob: device::GPIOB,
    gpiod: device::GPIOD,
    gpioe: device::GPIOE,
}

impl Bus {
    pub fn new(pins: Pins) -> Self {
        let peripherals = unsafe { device::Peripherals::steal() };

        Self {
            _pins: pins,
            gpiob: peripherals.GPIOB,
            gpiod: peripherals.GPIOD,
            gpioe: peripherals.GPIOE,
        }
    }

    pub fn read(&self) -> Action {
        // Read port data
        let pb = self.gpiob.idr.read().bits() as u16;
        let pd = self.gpiod.idr.read().bits() as u16;
        let pe = self.gpioe.idr.read().bits() as u16;

        // Read control signals
        // let ctrli_0 = pd & (1 << 2) == 0; // Read CTRLI_0 from PD2
        // let ctrli_1 = pb & (1 << 8) == 0; // Read CTRLI_1 from PB8
        let rsti = pb & (1 << 9) == 0; // Read RSTI from PB9
        let dtei = pb & (1 << 14) == 0; // Read DTEI from PB14

        if rsti {
            Action::Reset
        } else if dtei {
            Action::Stop
        } else {
            // Read data line
            let mut payload = (pb >> 7) & 1; // Read data bit 0 from PB7.
            payload |= pe & (1 << 1); // Read data bit 1 from PE1
            payload |= (pe & 1) << 2; // Read data bit 2 from PE0
            payload |= (pe & (1 << 2)) << 1; // Read data bit 3 from PE2
            payload |= (pe & (1 << 3)) << 1; // Read data bit 4 from PE3
            payload |= pe & (1 << 5); // Read data bit 5 from PE5
            payload |= (pe & (1 << 4)) << 2; // Read data bit 6 from PE4
            payload |= (pe & (1 << 6)) << 1; // Read data bit 7 from PE6
            payload |= (pb & (1 << 4)) << 4; // Read data bit 8 from PB4
            payload |= (pd & (1 << 6)) << 3; // Read data bit 9 from PD6
            payload |= (pd & (1 << 5)) << 5; // Read data bit 10 from PD5
            payload |= (pd & (1 << 7)) << 4; // Read data bit 11 from PD7
            payload |= (pd & (1 << 4)) << 8; // Read data bit 12 from PD4
            payload |= (pd & (1 << 1)) << 12; // Read data bit 13 from PD1
            payload |= (pd & (1 << 3)) << 11; // Read data bit 14 from PD3
            payload |= (pd & 1) << 15; // Read data bit 15 from PD0
            payload ^= u16::MAX; // Flip bits to convert from logical level 0 to 1
            Action::Data(payload)
        }
    }
}
