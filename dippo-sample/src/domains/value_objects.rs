pub mod user_email;
pub mod user_id;
pub mod user_name;
use std::fmt::Display;

pub trait ValueObject<T>: Display + PartialEq {
    fn value(&self) -> &T;
}
