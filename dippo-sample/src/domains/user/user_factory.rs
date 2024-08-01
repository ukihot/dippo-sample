use crate::domains::user::user_entity::User;
use crate::domains::value_objects::user_email::UserEmail;
use crate::domains::value_objects::user_id::UserId;
use crate::domains::value_objects::user_name::UserName;

pub trait UserFactory {
    fn create(&self, id: u32, name: String, email: String) -> Result<User, String>;
}

pub struct DefaultUserFactory;

impl UserFactory for DefaultUserFactory {
    fn create(&self, id: u32, name: String, email: String) -> Result<User, String> {
        let user_id = UserId::new(id).map_err(|_| "Invalid UserId".to_string())?;
        let user_name = UserName::new(name).map_err(|_| "Invalid UserName".to_string())?;
        let user_email = UserEmail::new(email).map_err(|_| "Invalid UserEmail".to_string())?;

        Ok(User::new(user_id, user_name, user_email))
    }
}
