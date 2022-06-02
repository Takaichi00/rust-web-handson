use std::{sync::Arc};

use rust_web_handson_domain::{repository::RepositoriesModuleExt, model::todo::Todo};
use rust_web_handson_infra::{
    modules::{RepositoriesModule},
};

use crate::usecase::{todo::{TodoUseCase}, UseCaseImpl};

use mockall::automock;

pub struct UseCaseModules {
    todo_usecase: UseCaseImpl<Todo, RepositoriesModule>,
}

pub trait UseCaseModulesExt {
    type TodoUc: TodoUseCase;
    fn todo_usecase(&self) -> &Self::TodoUc;
}

impl UseCaseModules {
    pub async fn new() -> Self {
        // initialize middlewares
        let repositories = Arc::new(RepositoriesModule::new().await);

        // make usecase instances
        let todo_usecase = UseCaseImpl::new(repositories.clone());

        // make di container
        Self {
            todo_usecase
        }
    }
}

#[automock(type TodoUc=MockTodoUseCase;)]
impl UseCaseModulesExt for UseCaseModules {
    type TodoUc = UseCaseImpl<Todo, RepositoriesModule>;
    fn todo_usecase(&self) -> &Self::TodoUc {
        &self.todo_usecase
    }
}
