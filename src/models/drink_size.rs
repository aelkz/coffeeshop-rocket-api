use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DrinkSize {
    Small,
    Medium,
    Large,
    Standard,
}

// Implement FromStr for conversion from string
impl FromStr for DrinkSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "small" => Ok(DrinkSize::Small),
            "medium" => Ok(DrinkSize::Medium),
            "large" => Ok(DrinkSize::Large),
            "standard" => Ok(DrinkSize::Standard),
            _ => Err(format!("'{}' is not a valid DrinkSize", s)),
        }
    }
}

// Implement Display for conversion to string
impl fmt::Display for DrinkSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrinkSize::Small => write!(f, "small"),
            DrinkSize::Medium => write!(f, "medium"),
            DrinkSize::Large => write!(f, "large"),
            DrinkSize::Standard => write!(f, "standard"),
        }
    }
}