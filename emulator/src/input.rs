use stm32f1xx_hal::{device, gpio};

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub enum Frame {
    Data(u16),
    Set,
    Reset,
    End,
}

pub struct Pins {
    pub do_0: Pin<'D', 0>,
    pub do_1: Pin<'D', 1>,
    pub do_2: Pin<'D', 2>,
    pub do_3: Pin<'D', 3>,
    pub do_4: Pin<'D', 4>,
    pub do_5: Pin<'D', 5>,
    pub do_6: Pin<'D', 6>,
    pub do_7: Pin<'E', 11>,
    pub do_8: Pin<'A', 8>,
    pub do_9: Pin<'C', 6>,
    pub do_10: Pin<'C', 7>,
    pub do_11: Pin<'D', 11>,
    pub do_12: Pin<'D', 12>,
    pub do_13: Pin<'D', 13>,
    pub do_14: Pin<'D', 14>,
    pub do_15: Pin<'D', 15>,
    pub ctrlo_0: Pin<'B', 15>,
    pub ctrlo_1: Pin<'B', 14>,
    pub ctrld: Pin<'B', 10>,
    pub rste: Pin<'E', 14>,
    pub sete: Pin<'E', 13>,
    pub dteo: Pin<'E', 12>,
    // RDY and ERRO are configured as interrupts
}

pub struct Bus {
    _pins: Pins,
    gpioa: device::GPIOA,
    gpiob: device::GPIOB,
    gpioc: device::GPIOC,
    gpiod: device::GPIOD,
    gpioe: device::GPIOE,
}

impl Bus {
    pub fn new(pins: Pins) -> Self {
        let periph = unsafe { device::Peripherals::steal() };

        Self {
            _pins: pins,
            gpioa: periph.GPIOA,
            gpiob: periph.GPIOB,
            gpioc: periph.GPIOC,
            gpiod: periph.GPIOD,
            gpioe: periph.GPIOE,
        }
    }

    pub fn read(&self) -> Frame {
        let mask = self.read_mask();
        Frame::from(mask.reverse_bits())
    }

    fn read_mask(&self) -> DataMask {
        DataMask {
            gpioa: self.gpioa.idr.read().bits(),
            gpiob: self.gpiob.idr.read().bits(),
            gpioc: self.gpioc.idr.read().bits(),
            gpiod: self.gpiod.idr.read().bits(),
            gpioe: self.gpioe.idr.read().bits(),
        }
    }
}

struct DataMask {
    pub gpioa: u32,
    pub gpiob: u32,
    pub gpioc: u32,
    pub gpiod: u32,
    pub gpioe: u32,
}

impl DataMask {
    pub fn reverse_bits(self) -> Self {
        Self {
            gpioa: self.gpioa ^ u32::MAX,
            gpiob: self.gpiob ^ u32::MAX,
            gpioc: self.gpioc ^ u32::MAX,
            gpiod: self.gpiod ^ u32::MAX,
            gpioe: self.gpioe ^ u32::MAX,
        }
    }

    pub fn dteo(&self) -> bool {
        (self.gpioe >> 12) & 1 == 1 // read DTEO from PE12
    }

    pub fn sete(&self) -> bool {
        (self.gpioe >> 13) & 1 == 1 // read SETE from PE13
    }

    pub fn rste(&self) -> bool {
        (self.gpioe >> 14) & 1 == 1 // read RSTE from PE14
    }

    pub fn data(&self) -> u16 {
        let mut data = self.gpiod & 0b1111111; // read bits 0..6 from PD[0..6]
        data |= (self.gpioe & (1 << 11)) >> 4; // read bit 7 from PE11
        data |= self.gpioa & (1 << 8); // read bit 8 from PA8
        data |= (self.gpioc & (0b11 << 6)) << 3; // read bit 9..10 from PC[6..7]
        data |= self.gpiod & (0b11111 << 11); // read bit 11..15 from PD[11..15]
        data as u16
    }
}

impl From<DataMask> for Frame {
    fn from(value: DataMask) -> Self {
        // let erro = (1 << 15) & value.gpioe != 0; // read ERRO from PE15
        // let ctrld = (value.gpiob >> 10) & 1 == 1; // read CTRDL from BP10
        // let ctrlo_1 = (value.gpiob >> 14) & 1 == 1; // read CTRLO_1 from BP14
        // let ctrlo_0 = (value.gpiob >> 15) & 1 == 1; // read CTRLO_0 from BP15

        if value.rste() {
            Self::Reset
        } else if value.sete() {
            Self::Set
        } else if value.dteo() {
            Self::End
        } else {
            Self::Data(value.data())
        }
    }
}
