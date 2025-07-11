use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Paid,
    Preparing,
    Ready,
    Completed,
    Cancelled,
}

impl FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(OrderStatus::Pending),
            "paid" => Ok(OrderStatus::Paid),
            "preparing" => Ok(OrderStatus::Preparing),
            "ready" => Ok(OrderStatus::Ready),
            "completed" => Ok(OrderStatus::Completed),
            "cancelled" => Ok(OrderStatus::Cancelled),
            _ => Err(format!("'{}' is not a valid OrderStatus", s)),
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Paid => write!(f, "paid"),
            OrderStatus::Preparing => write!(f, "preparing"),
            OrderStatus::Ready => write!(f, "ready"),
            OrderStatus::Completed => write!(f, "completed"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}