mod applications;
mod domains;
mod infrastructures;
mod presentations;
//mod dippotamus;

use domains::user::user_factory::DefaultUserFactory;

use crate::applications::user_usecase::user_interactor::UserInteractor;
use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
use crate::presentations::controller::UserController;

use crate::presentations::presenter::UserPresenter;

fn main() {
    //let container = initialize_di();
    // Create instances of dependencies
    let user_repository = UserRepositoryImpl;
    let user_presenter = UserPresenter;
    let user_factory = DefaultUserFactory;
    let interactor = UserInteractor::new(user_repository, user_presenter, user_factory);
    let controller = UserController::new(interactor);

    // User registration process
    controller.register(1, "Alice".to_string(), "alice@grillware.com".to_string());
}
