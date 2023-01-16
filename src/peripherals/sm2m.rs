use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct OutputBus<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15> {
    pub bit0: B0,
    pub bit1: B1,
    pub bit2: B2,
    pub bit3: B3,
    pub bit4: B4,
    pub bit5: B5,
    pub bit6: B6,
    pub bit7: B7,
    pub bit8: B8,
    pub bit9: B9,
    pub bit10: B10,
    pub bit11: B11,
    pub bit12: B12,
    pub bit13: B13,
    pub bit14: B14,
    pub bit15: B15,
}

impl<
        B0: OutputPin,
        B1: OutputPin,
        B2: OutputPin,
        B3: OutputPin,
        B4: OutputPin,
        B5: OutputPin,
        B6: OutputPin,
        B7: OutputPin,
        B8: OutputPin,
        B9: OutputPin,
        B10: OutputPin,
        B11: OutputPin,
        B12: OutputPin,
        B13: OutputPin,
        B14: OutputPin,
        B15: OutputPin,
    > OutputBus<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15>
{
    pub fn new(
        bit0: B0,
        bit1: B1,
        bit2: B2,
        bit3: B3,
        bit4: B4,
        bit5: B5,
        bit6: B6,
        bit7: B7,
        bit8: B8,
        bit9: B9,
        bit10: B10,
        bit11: B11,
        bit12: B12,
        bit13: B13,
        bit14: B14,
        bit15: B15,
    ) -> Self {
        Self {
            bit0,
            bit1,
            bit2,
            bit3,
            bit4,
            bit5,
            bit6,
            bit7,
            bit8,
            bit9,
            bit10,
            bit11,
            bit12,
            bit13,
            bit14,
            bit15,
        }
    }

    pub fn write(&mut self, value: u16) {
        fn write_bit<P: OutputPin>(pin: &mut P, value: u16, index: u8) {
            if value & 1 << index != 0 {
                pin.set_high().ok();
            } else {
                pin.set_low().ok();
            }
        }

        write_bit(&mut self.bit0, value, 0);
        write_bit(&mut self.bit1, value, 1);
        write_bit(&mut self.bit2, value, 2);
        write_bit(&mut self.bit3, value, 3);
        write_bit(&mut self.bit4, value, 4);
        write_bit(&mut self.bit5, value, 5);
        write_bit(&mut self.bit6, value, 6);
        write_bit(&mut self.bit7, value, 7);
        write_bit(&mut self.bit8, value, 8);
        write_bit(&mut self.bit9, value, 9);
        write_bit(&mut self.bit10, value, 10);
        write_bit(&mut self.bit11, value, 11);
        write_bit(&mut self.bit12, value, 12);
        write_bit(&mut self.bit13, value, 13);
        write_bit(&mut self.bit14, value, 14);
        write_bit(&mut self.bit15, value, 15);
    }
}

pub struct InputBus<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15> {
    pub bit0: B0,
    pub bit1: B1,
    pub bit2: B2,
    pub bit3: B3,
    pub bit4: B4,
    pub bit5: B5,
    pub bit6: B6,
    pub bit7: B7,
    pub bit8: B8,
    pub bit9: B9,
    pub bit10: B10,
    pub bit11: B11,
    pub bit12: B12,
    pub bit13: B13,
    pub bit14: B14,
    pub bit15: B15,
}

impl<
        B0: InputPin,
        B1: InputPin,
        B2: InputPin,
        B3: InputPin,
        B4: InputPin,
        B5: InputPin,
        B6: InputPin,
        B7: InputPin,
        B8: InputPin,
        B9: InputPin,
        B10: InputPin,
        B11: InputPin,
        B12: InputPin,
        B13: InputPin,
        B14: InputPin,
        B15: InputPin,
    > InputBus<B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15>
{
    pub fn new(
        bit0: B0,
        bit1: B1,
        bit2: B2,
        bit3: B3,
        bit4: B4,
        bit5: B5,
        bit6: B6,
        bit7: B7,
        bit8: B8,
        bit9: B9,
        bit10: B10,
        bit11: B11,
        bit12: B12,
        bit13: B13,
        bit14: B14,
        bit15: B15,
    ) -> Self {
        Self {
            bit0,
            bit1,
            bit2,
            bit3,
            bit4,
            bit5,
            bit6,
            bit7,
            bit8,
            bit9,
            bit10,
            bit11,
            bit12,
            bit13,
            bit14,
            bit15,
        }
    }

    pub fn read(&self) -> u16 {
        fn read_bit<P: InputPin>(pin: &P) -> u16 {
            pin.is_high().map(|value| value as u16).unwrap_or_default()
        }

        let mut word = 0;
        word &= read_bit(&self.bit0) << 0;
        word &= read_bit(&self.bit1) << 1;
        word &= read_bit(&self.bit2) << 2;
        word &= read_bit(&self.bit3) << 3;
        word &= read_bit(&self.bit4) << 4;
        word &= read_bit(&self.bit5) << 5;
        word &= read_bit(&self.bit6) << 6;
        word &= read_bit(&self.bit7) << 7;
        word &= read_bit(&self.bit8) << 8;
        word &= read_bit(&self.bit9) << 9;
        word &= read_bit(&self.bit10) << 10;
        word &= read_bit(&self.bit11) << 11;
        word &= read_bit(&self.bit12) << 12;
        word &= read_bit(&self.bit13) << 13;
        word &= read_bit(&self.bit14) << 14;
        word &= read_bit(&self.bit15) << 15;
        word
    }
}
