use core::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct InputData(u32);

impl InputData {
    pub fn invert(self) -> Self {
        Self(self.0 ^ u32::MAX)
    }

    pub fn payload(&self) -> u16 {
        self.0 as u16
    }

    pub fn is_ctrli_0_set(&self) -> bool {
        (self.0 & 0x10000) >> 16 == 1
    }

    pub fn is_ctrli_1_set(&self) -> bool {
        (self.0 & 0x20000) >> 17 == 1
    }

    pub fn is_rsti_set(&self) -> bool {
        (self.0 & 0x40000) >> 18 == 1
    }

    // Used as interrupt
    // pub fn is_dtsi_set(&self) -> bool {
    //     (self.0 & 0x80000) >> 19 == 1
    // }

    // Not used in favor of DTSI
    // pub fn is_dtli_set(&self) -> bool {
    //     (self.0 & 0x100000) >> 20 == 1
    // }

    pub fn is_dtei_set(&self) -> bool {
        (self.0 & 0x200000) >> 21 == 1
    }
}

impl From<u32> for InputData {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for InputData {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InputData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait SM2MBusRead {
    fn read(&self, data: InputData) -> InputData;
}

#[derive(Default)]
pub struct OutputData(u32);

impl OutputData {
    pub fn invert(self) -> Self {
        Self(self.0 ^ u32::MAX)
    }

    pub fn with_payload(mut self, payload: u16) -> Self {
        Self(self.0 | payload as u32) // set all 16 bit
    }

    pub fn with_ctrlo_0(mut self) -> Self {
        Self(self.0 | 0x10000) // set bit 16
    }

    pub fn with_ctrlo_1(mut self) -> Self {
        Self(self.0 | 0x20000) // set bit 17
    }

    pub fn with_rdy(mut self) -> Self {
        Self(self.0 | 0x40000) // set bit 18
    }

    pub fn with_ctrld(mut self) -> Self {
        Self(self.0 | 0x80000) // set bit 19
    }

    pub fn with_erro(mut self) -> Self {
        Self(self.0 | 0x100000) // set bit 20
    }

    pub fn with_rste(mut self) -> Self {
        Self(self.0 | 0x200000) // set bit 21
    }

    pub fn with_sete(mut self) -> Self {
        Self(self.0 | 0x400000) // set bit 22
    }

    pub fn with_dteo(mut self) -> Self {
        Self(self.0 | 0x800000) // set bit 23
    }
}

impl Deref for OutputData {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait SM2MBusWrite {
    fn write(&mut self, data: &OutputData);
}
