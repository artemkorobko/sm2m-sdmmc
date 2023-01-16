use embedded_sdmmc::{TimeSource, Timestamp};

#[derive(Default)]
pub struct StaticTimeSource {}

impl TimeSource for StaticTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 53,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}
