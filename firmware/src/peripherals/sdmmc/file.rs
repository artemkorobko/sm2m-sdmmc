use heapless::String;

pub type FileName = String<5>;

pub trait AsFileName {
    fn as_file_name(self) -> String<5>;
}

impl AsFileName for u16 {
    fn as_file_name(self) -> String<5> {
        const MAX_CHARS: usize = 10;
        const CHARS: [char; MAX_CHARS] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut file_name = String::new();
        let mut num = self as usize;

        while num > 0 {
            let digit = core::cmp::min(MAX_CHARS - 1, num % 10);
            let symbol = CHARS[digit];
            file_name.push(symbol).ok();
            num /= 10;
        }

        file_name
    }
}
