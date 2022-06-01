use std::{sync::Arc};

use rust_web_handson_infra::{
    client::mysql::Rds,
    modules::{RepositoriesModule},
};

use crate::usecase::todo::{TodoUseCase};

use mockall::automock;

pub struct UseCaseModules {
    todo_usecase: TodoUseCase<RepositoriesModule>,
    todo_create_usecase: TodoUseCase<RepositoriesModule>,
    // todo_create_try_usecase: TodoUseCase<>
}

#[automock]
impl UseCaseModules {
    pub async fn new(rds: Rds) -> Self {
        // initialize middlewares
        let repositories = Arc::new(RepositoriesModule::new(rds.clone()));

        // make usecase instances
        let todo_usecase = TodoUseCase::new(repositories.clone());
        let todo_create_usecase = TodoUseCase::new(repositories.clone());


        // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        // let todo_create_try_usecase = TodoUseCase::new();

        // make di container
        Self {
            todo_usecase,
            todo_create_usecase,
            // todo_create_try_usecase,
        }
    }

    pub fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_usecase
    }

    pub fn todo_create_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_create_usecase
    }
}
