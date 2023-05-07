use stm32f1xx_hal::{device, gpio};

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub enum Frame {
    Ack(u16),
    Error(u16),
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
    pub erro: Pin<'E', 15>,
    pub rste: Pin<'E', 14>,
    pub sete: Pin<'E', 13>,
    pub dteo: Pin<'E', 12>,
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
        Frame::from(mask)
    }

    pub fn read_reversed(&self) -> Frame {
        let mask = self.read_mask();
        Frame::from(mask.reverse_bits())
    }

    fn read_mask(&self) -> DataMask {
        DataMask {
            gpioa: self.gpioa.idr.read().bits() as u16,
            gpiob: self.gpiob.idr.read().bits() as u16,
            gpioc: self.gpioc.idr.read().bits() as u16,
            gpiod: self.gpiod.idr.read().bits() as u16,
            gpioe: self.gpioe.idr.read().bits() as u16,
        }
    }
}

struct DataMask {
    pub gpioa: u16,
    pub gpiob: u16,
    pub gpioc: u16,
    pub gpiod: u16,
    pub gpioe: u16,
}

impl DataMask {
    pub fn reverse_bits(self) -> Self {
        Self {
            gpioa: self.gpioa ^ u16::MAX,
            gpiob: self.gpiob ^ u16::MAX,
            gpioc: self.gpioc ^ u16::MAX,
            gpiod: self.gpiod ^ u16::MAX,
            gpioe: self.gpioe ^ u16::MAX,
        }
    }
}

impl From<DataMask> for Frame {
    fn from(value: DataMask) -> Self {
        let dteo = (value.gpioe >> 12) & 1 == 1; // read DTEO from PE12
        let sete = (value.gpioe >> 13) & 1 == 1; // read SETE from PE13
        let rste = (value.gpioe >> 14) & 1 == 1; // read RSTE from PE14
        let erro = (1 << 15) & value.gpioe != 0; // read ERRO from PE15
        let ctrld = (value.gpiob >> 10) & 1 == 1; // read CTRDL from BP10
        let ctrlo_1 = (value.gpiob >> 14) & 1 == 1; // read CTRLO_1 from BP14
        let ctrlo_0 = (value.gpiob >> 15) & 1 == 1; // read CTRLO_0 from BP15

        let mut data = value.gpiob & 0b1111111; // read bits 0..6 from PD[0..6]
        data |= (value.gpioe & 0b100000000000) >> 4; // read bit 7 from PE11
        data |= value.gpioa & 0b100000000; // read bit 8 from PA8
        data |= (value.gpioc & 0b11000000) << 3; // read bit 9..10 from PC[6..7]
        data |= (value.gpiod & 0b1111100000000000) << 3; // read bit 11..15 from PD[11..15]

        // defmt::println!(
        //     "dteo: {}, sete: {}, rste: {}, erro: {}, ctrld: {}, ctrlo_1: {}, ctrlo_0: {}",
        //     dteo,
        //     sete,
        //     rste,
        //     erro,
        //     ctrld,
        //     ctrlo_1,
        //     ctrlo_0,
        // );
        // defmt::println!("Data: {}", data);

        if erro {
            Self::Error(data)
        } else if rste {
            Self::Reset
        } else if sete {
            Self::Set
        } else if dteo {
            Self::End
        } else {
            Self::Ack(data)
        }
    }
}
