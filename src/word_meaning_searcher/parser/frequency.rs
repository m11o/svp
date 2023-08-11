use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Frequency {
    Low,
    Middle,
    High,
    VeryHigh
}

impl Frequency {
    pub fn str2enum(frequency: &str) -> Frequency {
        match frequency {
            "very high" => Frequency::VeryHigh,
            "high"      => Frequency::High,
            "middle"    => Frequency::Middle,
            "low"       => Frequency::Low,
            _           => Frequency::Low
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
