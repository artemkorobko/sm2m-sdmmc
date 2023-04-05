use alloc::string::String;

pub trait AsFileName {
    fn as_file_name(&self) -> String;
}

impl AsFileName for u16 {
    fn as_file_name(&self) -> String {
        const MAX_CHARS: usize = 10;
        const CHARS: [char; MAX_CHARS] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut file_name = String::with_capacity(5);
        let mut num = *self as usize;

        while num > 0 {
            let digit = core::cmp::min(MAX_CHARS - 1, num % 10);
            let symbol = CHARS[digit];
            file_name.push(symbol);
            num /= 10;
        }

        file_name
    }
}
