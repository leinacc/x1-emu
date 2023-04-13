pub struct RTC {
    pub day: u8, // BCD day in month
    pub month: u8, // 1-idxed month
    pub weekday: u8,
    pub year: u8, // BCD 2-digit year
    pub hour: u8, // BCD
    pub minute: u8, // BCD
    pub second: u8, // BCD
}

impl RTC {
    pub fn new() -> Self {
        Self {
            day: 0,
            month: 1,
            weekday: 0,
            year: 0,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}
