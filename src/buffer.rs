use core::ops::{Deref, DerefMut};

pub const BUFFER_SIZE: usize = 1024 * 10;

pub type InnerBuffer = heapless::Vec<u8, BUFFER_SIZE>;

pub struct Buffer(InnerBuffer);

impl Deref for Buffer {
    type Target = InnerBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
