use crate::domains::value_objects::ValueObject;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct UserId {
    value: u32,
}

impl UserId {
    // `new` method now returns `Result<UserId, &'static str>` for validation
    pub fn new(value: u32) -> Result<Self, &'static str> {
        if value > 0 {
            Ok(UserId { value })
        } else {
            Err("UserId must be zero or greater")
        }
    }
}

impl ValueObject<u32> for UserId {
    fn value(&self) -> &u32 {
        &self.value
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
