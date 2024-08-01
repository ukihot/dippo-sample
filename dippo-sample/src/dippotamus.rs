use dippo::DippotamusContainer;

use crate::applications::user_usecase::user_input_port::UserInputPort;
use crate::applications::user_usecase::user_interactor::UserInteractor;
use crate::applications::user_usecase::user_output_port::UserOutputPort;
use crate::domains::user::user_factory::DefaultUserFactory;
use crate::domains::user::user_factory::UserFactory;
use crate::domains::user::user_repository::UserRepository;
use crate::infrastructures::user_repository_impl::UserRepositoryImpl;
use crate::presentations::presenter::UserPresenter;

pub fn initialize() -> DippotamusContainer {
    let mut container = DippotamusContainer::new();

    // サービスをためこむ
    container.stockpile::<dyn UserRepository>(UserRepositoryImpl);
    container.stockpile::<dyn UserOutputPort>(UserPresenter);
    container.stockpile::<dyn UserInputPort>(UserInteractor);
    container.stockpile::<dyn UserFactory>(DefaultUserFactory);

    container
}
