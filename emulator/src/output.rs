use stm32f1xx_hal::{device, gpio};

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Output<gpio::PushPull>>;

macro_rules! port_write {
    ($GPIO:expr, $CLR_MASK:expr, $BIT_MASK:expr) => {
        // let gpio = stringify!($GPIO);
        $GPIO.odr.modify(|r, w| {
            let mut bits = r.bits(); // read bits
            // defmt::println!(
            //     "{} bits {:#034b}, clr_mask: {:#034b}, bit_mask: {:#034b}",
            //     gpio,
            //     bits,
            //     $CLR_MASK,
            //     $BIT_MASK
            // );
            bits &= $CLR_MASK; // clear bits before applying mask
            bits |= $BIT_MASK & ($CLR_MASK ^ 0xFFFF); // apply mask
            // defmt::println!("{} result {:#034b}", gpio, bits);
            unsafe { w.bits(bits) } // write bits
        });
    };
}

pub enum Frame {
    Reset,
    CheckStatus,
    Address(u16),
    Write(u16),
    Read,
    Stop,
}

pub struct Pins {
    pub di_0: Pin<'B', 4>,
    pub di_1: Pin<'B', 5>,
    pub di_2: Pin<'B', 6>,
    pub di_3: Pin<'B', 7>,
    pub di_4: Pin<'B', 8>,
    pub di_5: Pin<'B', 9>,
    pub di_6: Pin<'E', 2>,
    pub di_7: Pin<'E', 3>,
    pub di_8: Pin<'E', 4>,
    pub di_9: Pin<'E', 5>,
    pub di_10: Pin<'E', 6>,
    pub di_11: Pin<'C', 13>,
    pub di_12: Pin<'C', 0>,
    pub di_13: Pin<'C', 2>,
    pub di_14: Pin<'C', 3>,
    pub di_15: Pin<'A', 0>,
    pub ctrli_0: Pin<'A', 3>, // temprarly ignored
    pub ctrli_1: Pin<'A', 5>, // temprarly ignored
    pub dtsi: Pin<'A', 6>,    // ignored in favour of DTLI
    pub dtli: Pin<'A', 7>,
    pub dtei: Pin<'C', 4>,
    pub rsti: Pin<'C', 5>,
}

pub struct Bus {
    pins: Pins,
    gpioa: device::GPIOA,
    gpiob: device::GPIOB,
    gpioc: device::GPIOC,
    gpioe: device::GPIOE,
}

impl Bus {
    pub fn new(pins: Pins) -> Self {
        let periph = unsafe { device::Peripherals::steal() };

        let mut bus = Self {
            pins,
            gpioa: periph.GPIOA,
            gpiob: periph.GPIOB,
            gpioc: periph.GPIOC,
            gpioe: periph.GPIOE,
        };

        bus.write_mask(DataMask::default());
        bus
    }

    pub fn write(&mut self, frame: Frame) {
        let mask = DataMask::from(frame);
        self.pins.dtli.set_low();
        self.write_mask(mask);
        self.pins.dtli.set_high();
    }

    pub fn write_reversed(&mut self, frame: Frame) {
        let mask = DataMask::from(frame);
        self.pins.dtli.set_high();
        self.write_mask(mask.reverse_bits());
        self.pins.dtli.set_low();
    }

    fn write_mask(&mut self, mask: DataMask) {
        port_write!(self.gpioa, 0b1111111100010110, mask.gpioa as u32);
        port_write!(self.gpiob, 0b1111110000001111, mask.gpiob as u32);
        port_write!(self.gpioc, 0b1101111111000010, mask.gpioc as u32);
        port_write!(self.gpioe, 0b1111111110000011, mask.gpioe as u32);
    }
}

#[derive(Default)]
struct DataMask {
    pub gpioa: u16,
    pub gpiob: u16,
    pub gpioc: u16,
    pub gpioe: u16,
}

impl DataMask {
    pub fn reverse_bits(self) -> Self {
        Self {
            gpioa: self.gpioa ^ u16::MAX,
            gpiob: self.gpiob ^ u16::MAX,
            gpioc: self.gpioc ^ u16::MAX,
            gpioe: self.gpioe ^ u16::MAX,
        }
    }
}

impl From<Frame> for DataMask {
    fn from(value: Frame) -> Self {
        match value {
            Frame::Reset => Self {
                gpioa: 0,
                gpiob: 0,
                gpioc: (1 << 5), // set RSTI bit to pc5
                gpioe: 0,
            },
            Frame::Address(addr) => {
                let mut gpioc = (addr & (1 << 1)) << 12; // set address bit 1 to pc13
                gpioc |= (addr & (1 << 2)) >> 2; // set address bit 2 to pc0
                gpioc |= (addr & (1 << 3)) >> 1; // set address bit 3 to pc2
                gpioc |= (addr & (1 << 4)) >> 1; // set address bit 4 to pc3

                Self {
                    gpioa: (addr & (1 << 6)) >> 6, // set address bit 6 to pa0
                    gpiob: 0b0011 << 4,            // set command bits [0..4] to 0011 to pb[4..7]
                    gpioc,
                    gpioe: (addr & (1 << 0)) << 6, // set address bit 0 to pe6
                }
            }
            Frame::Write(data) => {
                let mut gpiob = (data & (1 << 0)) << 4; // set data bit 0 to pb4
                gpiob |= (data & (1 << 1)) << 4; // set data bit 1 to pb5
                gpiob |= (data & (1 << 2)) << 4; // set data bit 2 to pb6
                gpiob |= (data & (1 << 3)) << 4; // set data bit 3 to pb7
                gpiob |= (data & (1 << 4)) << 4; // set data bit 4 to pb8
                gpiob |= (data & (1 << 5)) << 4; // set data bit 5 to pb9

                let mut gpioc = (data & (1 << 11)) << 2; // set data bit 11 to pc13
                gpioc |= (data & (1 << 12)) >> 12; // set data bit 12 to pc0
                gpioc |= (data & (1 << 13)) >> 11; // set data bit 13 to pc2
                gpioc |= (data & (1 << 14)) >> 11; // set data bit 14 to pc3

                let mut gpioe = (data & (1 << 6)) >> 4; // set data bit 6 to pe2
                gpioe |= (data & (1 << 7)) >> 4; // set data bit 7 to pe3
                gpioe |= (data & (1 << 8)) >> 4; // set data bit 8 to pe4
                gpioe |= (data & (1 << 9)) >> 4; // set data bit 9 to pe5
                gpioe |= (data & (1 << 10)) >> 4; // set data bit 10 to pe6

                Self {
                    gpioa: (data & (1 << 15)) >> 15, // set data bit 15 to pa0
                    gpiob,
                    gpioc,
                    gpioe,
                }
            }
            Frame::CheckStatus | Frame::Read => Self {
                gpioa: 0,
                gpiob: 0,
                gpioc: 0,
                gpioe: 0,
            },
            Frame::Stop => Self {
                gpioa: 0,
                gpiob: 0,
                gpioc: 1 << 4, // set DTEI bit to pc4
                gpioe: 0,
            },
        }
    }
}
