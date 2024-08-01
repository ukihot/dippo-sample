use crate::domains::value_objects::ValueObject;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct UserEmail {
    value: String,
}

impl UserEmail {
    pub fn new(value: String) -> Result<Self, &'static str> {
        if value.contains('@') {
            Ok(UserEmail { value })
        } else {
            Err("Invalid email format")
        }
    }
}

impl ValueObject<String> for UserEmail {
    fn value(&self) -> &String {
        &self.value
    }
}

impl fmt::Display for UserEmail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
