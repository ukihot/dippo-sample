use crate::domains::{user::user_entity::User, value_objects::ValueObject};

pub struct UserDTO {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl From<&User> for UserDTO {
    fn from(user: &User) -> Self {
        UserDTO {
            id: *user.id.value(),
            name: (&user.name).to_string(),
            email: (&user.email).to_string(),
        }
    }
}
