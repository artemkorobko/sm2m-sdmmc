use stm32f1xx_hal::{device, gpio};

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub enum Frame {
    Reset,
    // CheckStatus, <- these should be handled based on mode or state
    // Address(u16),
    // Read,
    // Write,
    EndOfDataTransfer,
    Payload(u16),
}

// impl Frame {
//     pub fn from(value: Input) -> Result<Self, AppError> {
//         if value.data & 0x0F == 0x00 {
//             Ok(Self::CheckStatus)
//         } else if value.data & 0x0F == 0x01 {
//             Ok(Self::Write)
//         } else if value.data & 0x0F == 0x02 {
//             Ok(Self::Read)
//         } else if value.data & 0x0F == 0x03 {
//             Ok(Self::Address(0xFC00))
//         } else if value.rst {
//             Ok(Self::Reset)
//         } else {
//             Err(AppError::UnknownCommand)
//         }
//     }
// }

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

    pub fn read(&self) -> Frame {
        // Read port data
        let pb = self.gpiob.idr.read().bits() as u16;
        let pd = self.gpiod.idr.read().bits() as u16;
        let pe = self.gpioe.idr.read().bits() as u16;

        // Read control signals
        let ctrli_0 = pd & (1 << 2) == 0; // Read CTRLI_1 from PD2
        let ctrli_1 = pb & (1 << 8) == 0; // Read CTRL_1 from PB8
        let rsti = pb & (1 << 9) == 0; // Read RSTI from PB9
        let dtei = pb & (1 << 14) == 0; // Read DTEI from PB14

        if rsti {
            Frame::Reset
        } else if dtei {
            Frame::EndOfDataTransfer
        } else {
            // Read data line
            let mut data = (pb >> 7) & 1; // Read data bit 0 from PB7.
            data |= pe & (1 << 1); // Read data bit 1 from PE1
            data |= (pe & 1) << 2; // Read data bit 2 from PE0
            data |= (pe & (1 << 2)) << 1; // Read data bit 3 from PE2
            data |= (pe & (1 << 3)) << 1; // Read data bit 4 from PE3
            data |= pe & (1 << 5); // Read data bit 5 from PE5
            data |= (pe & (1 << 4)) << 2; // Read data bit 6 from PE4
            data |= (pe & (1 << 6)) << 1; // Read data bit 7 from PE6
            data |= (pb & (1 << 4)) << 4; // Read data bit 8 from PB4
            data |= (pd & (1 << 6)) << 3; // Read data bit 9 from PD6
            data |= (pd & (1 << 5)) << 5; // Read data bit 10 from PD5
            data |= (pd & (1 << 7)) << 4; // Read data bit 11 from PD7
            data |= (pd & (1 << 4)) << 8; // Read data bit 12 from PD4
            data |= (pd & (1 << 1)) << 12; // Read data bit 13 from PD1
            data |= (pd & (1 << 3)) << 11; // Read data bit 14 from PD3
            data |= (pd & 1) << 15; // Read data bit 15 from PD0
            data ^= u16::MAX; // Flip bits to convert from logical level 0 to 1
            Frame::Payload(data)
        }
    }
}
