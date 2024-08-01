mod applications;
mod dippotamus;
mod domains;
mod infrastructures;
mod presentations;

//use domains::user::user_factory::{self, DefaultUserFactory, UserFactory};
//use crate::applications::user_usecase::user_interactor::UserInteractor;
//use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
//use crate::presentations::presenter::UserPresenter;
use crate::applications::user_usecase::user_input_port::UserInputPort;
use crate::dippotamus::initialize_di;
use crate::presentations::controller::UserController;

fn main() {
    // Using Dependency Injection with Dippo
    let container = initialize_di();
    let user_interactor = container.spit_up::<dyn UserInputPort>().unwrap();
    let controller = UserController::new(user_interactor);

    // Without Dippo
    //let user_repository = UserRepositoryImpl;
    //let user_presenter = UserPresenter;
    //let user_factory = DefaultUserFactory;
    //let user_interactor = UserInteractor::new(user_repository, user_presenter, user_factory);
    //let controller = UserController::new(user_interactor);

    // User registration process
    controller.register(1, "Alice".to_string(), "alice@grillware.com".to_string());
}
