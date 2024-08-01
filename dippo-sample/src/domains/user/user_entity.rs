use crate::domains::value_objects::user_email::UserEmail;
use crate::domains::value_objects::user_id::UserId;
use crate::domains::value_objects::user_name::UserName;

pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: UserEmail,
}

impl User {
    pub fn new(id: UserId, name: UserName, email: UserEmail) -> Self {
        User { id, name, email }
    }
}
