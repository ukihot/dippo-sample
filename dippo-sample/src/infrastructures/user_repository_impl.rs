use crate::domains::user::user_entity::User;
use crate::domains::user::user_repository::UserRepository;

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn save(&self, user: &User) {
        println!("User saved: {} - {}", user.id, user.name);
    }
}
