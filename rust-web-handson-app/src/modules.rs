use std::{sync::Arc};

use rust_web_handson_infra::{
    client::mysql::Rds,
    modules::{RepositoriesModule, RepositoriesModuleExt},
};

use crate::usecase::todo::{TodoUseCase};

#[cfg(test)]
use mockall::automock;

// struct なので、mockall double を使う
pub struct UseCaseModules {
    todo_usecase: TodoUseCase<RepositoriesModule>,
    todo_create_usecase: TodoUseCase<RepositoriesModule>,
    // todo_create_try_usecase: TodoUseCase<>
}

#[cfg_attr(test, automock)]
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
}

// Ext とついているものが外部公開するする型
// Mock にするときは↓を Mock にするが、メソッド数分 mock にするのは大変
// これを解決するのが Mockall というライブラリ。TodoUseCase の空を返す。
// ↑ に加えて、expect_~~ というメソッドがはえる
// 空を返す struct は MockUseCaseModulesExt という命名になる
pub trait UseCaseModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule>;
    fn todo_create_usecase(&self) -> &TodoUseCase<RepositoriesModule>;
}

// こっちが内部利用用のモジュール
impl UseCaseModulesExt for UseCaseModules {
    type RepositoriesModule = RepositoriesModule;

    fn todo_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_usecase
    }

    // usecase の struct を渡してあげる
    // usecase のテストをする場合、UseCaseModulesExt の trait を実装してあげる必要がある
    // イメージ的には todo_create_usecase をオーバーライドすることでモックにすることができる
    fn todo_create_usecase(&self) -> &TodoUseCase<RepositoriesModule> {
        &self.todo_create_usecase
    }
}
