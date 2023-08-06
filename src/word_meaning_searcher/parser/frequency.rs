use std::fmt::{Display, Formatter, Result};

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

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Frequency::VeryHigh => write!(f, "very high"),
            Frequency::High => write!(f, "high"),
            Frequency::Middle => write!(f, "middle"),
            Frequency::Low => write!(f, "low")
        }
    }
}
