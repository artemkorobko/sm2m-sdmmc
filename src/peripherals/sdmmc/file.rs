use core::ops::Deref;

extern crate alloc;

pub const FILE_NAME_LEN: usize = 5;

#[derive(Clone)]
pub struct FileName(heapless::String<FILE_NAME_LEN>);

impl FileName {}

impl Deref for FileName {
    type Target = heapless::String<5>;

    fn deref(&self) -> &Self::Target {
        let string = alloc::string::String::new();
        &self.0
    }
}

pub trait AsFileName {
    fn as_file_name(&self) -> FileName;
}

impl AsFileName for u16 {
    fn as_file_name(&self) -> FileName {
        const MAX_CHARS: usize = 10;
        const CHARS: [char; MAX_CHARS] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut file_name = heapless::String::new();
        let mut num = *self as usize;

        while num > 0 {
            let digit = core::cmp::min(MAX_CHARS - 1, num % 10);
            let symbol = CHARS[digit];
            file_name.push(symbol).ok();
            num /= 10;
        }

        FileName(file_name)
    }
}
