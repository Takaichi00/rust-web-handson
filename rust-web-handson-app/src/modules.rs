use std::sync::Arc;

use rust_web_handson_infra::{
    client::mysql::Rds,
    modules::{RepositoriesModule, RepositoriesModuleExt},
};

use crate::usecase::todo::TodoUseCase;

pub struct UseCaseModules {
    todo_usecase: TodoUseCase<RepositoriesModule>,
}

impl UseCaseModules {
    pub async fn new(rds: Rds) -> Self {
        // initialize middlewares
        let repositories = Arc::new(RepositoriesModule::new(rds.clone()));

        // make usecase instances
        let todo_usecase = TodoUseCase::new(repositories.clone());

        // make di container
        Self { todo_usecase }
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
