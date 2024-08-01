use crate::applications::user_usecase::user_input_port::UserInputPort;

pub struct UserController<I: UserInputPort> {
    input_port: I,
}

impl<I: UserInputPort> UserController<I> {
    pub fn new(input_port: I) -> Self {
        UserController { input_port }
    }

    pub fn register(&self, id: u32, name: String, email: String) {
        self.input_port.register_user(id, name, email);
    }
}
