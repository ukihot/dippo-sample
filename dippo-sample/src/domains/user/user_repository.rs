use crate::domains::user::user_entity::User;

pub trait UserRepository {
    fn save(&self, user: &User);
}
