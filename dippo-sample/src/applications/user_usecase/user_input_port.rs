pub trait UserInputPort {
    fn register_user(&self, id: u32, name: String, email: String);
}
