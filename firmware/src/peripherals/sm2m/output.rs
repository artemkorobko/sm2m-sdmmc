use stm32f1xx_hal::{device, gpio};

macro_rules! port_write {
    ($GPIO:expr, $CLR_MASK:expr, $BIT_MASK:expr) => {
        $GPIO.odr.modify(|r, w| {
            let mut bits = r.bits(); // read bits
            bits &= $CLR_MASK; // clear bits before applying mask
            bits |= $BIT_MASK & ($CLR_MASK ^ 0xFFFF); // apply mask
            unsafe { w.bits(bits) } // write bits
        });
    };
}

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Output<gpio::PushPull>>;

pub enum Frame {
    Ack,
    Error(u16),
    Data(u16),
}

pub struct Pins {
    pub do_0: Pin<'C', 10>,
    pub do_1: Pin<'A', 12>,
    pub do_2: Pin<'A', 10>,
    pub do_3: Pin<'A', 8>,
    pub do_4: Pin<'C', 9>,
    pub do_5: Pin<'C', 8>,
    pub do_6: Pin<'C', 7>,
    pub do_7: Pin<'D', 15>,
    pub do_8: Pin<'D', 14>,
    pub do_9: Pin<'C', 6>,
    pub do_10: Pin<'D', 13>,
    pub do_11: Pin<'D', 8>,
    pub do_12: Pin<'D', 11>,
    pub do_13: Pin<'D', 12>,
    pub do_14: Pin<'B', 15>,
    pub do_15: Pin<'D', 9>,
    pub ctrlo_0: Pin<'A', 11>,
    pub ctrlo_1: Pin<'A', 9>,
    pub rdy: Pin<'D', 10>,
    pub ctrl_d: Pin<'C', 12>,
    pub erro: Pin<'A', 15>,
    pub rste: Pin<'B', 12>,
    pub sete: Pin<'C', 3>,
    pub dteo: Pin<'C', 11>,
}

pub struct Bus {
    pins: Pins,
    gpioa: device::GPIOA,
    gpiob: device::GPIOB,
    gpioc: device::GPIOC,
    gpiod: device::GPIOD,
}

const GPIOA_MASK: u32 = 0b0110000011111111;
const GPIOB_MASK: u32 = 0b0110111111111111;
const GPIOC_MASK: u32 = 0b1110000000110111;
const GPIOD_MASK: u32 = 0b0000010011111111;

impl Bus {
    pub fn new(pins: Pins) -> Self {
        let peripherals = unsafe { device::Peripherals::steal() };

        let mut bus = Self {
            pins,
            gpioa: peripherals.GPIOA,
            gpiob: peripherals.GPIOB,
            gpioc: peripherals.GPIOC,
            gpiod: peripherals.GPIOD,
        };

        bus.write_ack(); // Set default bus state.
        bus
    }

    pub fn write(&mut self, frame: Frame) {
        self.pins.rdy.set_high();

        // Assume that all signal pins CTRLO_0, CTRLO_1, RDY,
        // CTRL_D, ERRO, RSTE, SETE, DTEO, are set to 1 during write.
        match frame {
            Frame::Ack => {
                self.write_ack();
                self.pins.rdy.set_low();
            }
            Frame::Error(opcode) => {
                self.write_data(opcode);
                self.pins.erro.set_low();
            }
            Frame::Data(data) => {
                self.write_data(data);
                self.pins.rdy.set_low();
            }
        }
    }

    fn write_ack(&mut self) {
        port_write!(self.gpioa, GPIOA_MASK, u32::MAX); // Write 1 to pin 8, 9, 10, 11, 12, 15
        port_write!(self.gpiob, GPIOB_MASK, u32::MAX); // Write 1 to pin 12, 15
        port_write!(self.gpioc, GPIOC_MASK, u32::MAX); // Write 1 to pin 3, 6, 7, 8, 9, 10, 11, 12
        port_write!(self.gpiod, GPIOD_MASK, u32::MAX); // Write 1 to pin 8, 9, 10, 11, 12, 13, 14, 15
    }

    fn write_data(&self, data: u16) {
        let data = data as u32 ^ u32::MAX; // Flip bits to convert between logic levels

        let mut pa = 0b1000101000000000; // CTRLO_1 (PA9), CTRLO_0 (PA11) and ERRO (A15) are set to 1
        pa |= (data & (1 << 1)) << 11; // Write data bit 1 to PA12
        pa |= (data & (1 << 2)) << 8; // Write data bit 2 to PA10
        pa |= (data & (1 << 3)) << 5; // Write data bit 3 to PA8

        let mut pb = 0b0001000000000000; // RSTE (PB12) is set to 1
        pb |= (data & (1 << 14)) << 1; // Write data bit 14 to PB15

        let mut pc = 0b0001100000001000; // SETE (PC3), DTEO (PC11) and CTRL_D (PC12) are set to 1
        pc |= (data & 1) << 10; // Write data bit 0 to PC10
        pc |= (data & (1 << 4)) << 5; // Write data bit 4 to PC9
        pc |= (data & (1 << 5)) << 3; // Write data bit 5 to PC8
        pc |= (data & (1 << 6)) << 1; // Write data bit 6 to PC7
        pc |= (data & (1 << 9)) >> 3; // Write data bit 9 to PC6

        let mut pd = 0b0000010000000000; // RDY (PD10) is set to 1
        pc |= (data & (1 << 7)) << 8; // Write data bit 7 to PD15
        pd |= (data & (1 << 8)) << 6; // Write data bit 8 to PD14
        pd |= (data & (1 << 10)) << 3; // Write data bit 10 to PD13
        pd |= (data & (1 << 11)) >> 3; // Write data bit 11 to PD8
        pd |= (data & (1 << 12)) >> 1; // Write data bit 12 to PD11
        pd |= (data & (1 << 13)) >> 1; // Write data bit 13 to PD12
        pd |= (data & (1 << 15)) >> 6; // Write data bit 15 to PD9

        port_write!(self.gpioa, GPIOA_MASK, pa);
        port_write!(self.gpiob, GPIOB_MASK, pb);
        port_write!(self.gpioc, GPIOC_MASK, pc);
        port_write!(self.gpiod, GPIOD_MASK, pd);
    }
}
