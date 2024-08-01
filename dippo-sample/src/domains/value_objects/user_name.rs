use crate::domains::value_objects::ValueObject;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct UserName {
    value: String,
}

impl UserName {
    // `new` method now returns `Result<UserName, &'static str>` for validation
    pub fn new(value: String) -> Result<Self, &'static str> {
        if value.len() >= 2 {
            Ok(UserName { value })
        } else {
            Err("UserName must be at least 2 characters long")
        }
    }
}

impl ValueObject<String> for UserName {
    fn value(&self) -> &String {
        &self.value
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
