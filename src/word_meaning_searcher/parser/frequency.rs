#[derive(Debug, PartialEq)]
pub enum Frequency {
    Low,
    Middle,
    High,
    VeryHigh
}

impl Frequency {
    pub fn str2enum(frequency: String) -> Frequency {
        match frequency {
            String::from("very high") => Frequency::VeryHigh,
            String::from("high")      => Frequency::High,
            String::from("middle")    => Frequency::Middle,
            String::from("low")       => Frequency::Low
        }
    }
}
