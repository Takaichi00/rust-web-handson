use std::{sync::Arc, ptr::null};

use rust_web_handson_infra::{
    client::mysql::Rds,
    modules::{RepositoriesModule, RepositoriesModuleExt},
};

use crate::usecase::todo::{TodoUseCase, TodoCreateUseCase};

pub struct UseCaseModules {
    todo_usecase: TodoUseCase<RepositoriesModule>,
    // todo_create_usecase: TodoCreateUseCase<RepositoriesModule>
}

impl UseCaseModules {
    pub async fn new(rds: Rds) -> Self {
        // initialize middlewares
        let repositories = Arc::new(RepositoriesModule::new(rds.clone()));

        // make usecase instances
        let todo_usecase = TodoUseCase::new(repositories.clone());

        // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        // let todo_create_usecase = TodoCreateUseCase::new();

        // make di container
        Self {
            todo_usecase,
            // todo_create_usecase,
        }
    }
}

// Ext とついているものが外部公開するする型
pub trait UseCaseModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule>;
}

// こっちが内部利用用のモジュール
impl UseCaseModulesExt for UseCaseModules {
    type RepositoriesModule = RepositoriesModule;

    fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_usecase
    }
}
