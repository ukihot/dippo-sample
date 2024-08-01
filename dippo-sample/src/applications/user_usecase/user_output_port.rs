use crate::applications::dto::user_dto::UserDTO;

pub trait UserOutputPort {
    fn show_user(&self, user: &UserDTO);
}
