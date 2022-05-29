use std::{sync::Arc, ptr::null};

use derive_new::new;
use rust_web_handson_domain::{model::todo::Todo, repository::todo::TodoRepository};
use rust_web_handson_infra::modules::RepositoriesModuleExt;

#[derive(new)]
pub struct TodoUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

// 以下のようにもかけるが、実行時解析になってしまう
/**
 * pub struct TodoUseCase {
    repositories: Arc<dyn RepositoriesModuleExt>,
}
*/

#[derive(new)]
pub struct TodoCreateUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

#[derive(new)]
pub struct TodoCreateTryUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}



/*
 * ↓ Impl
 */


impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }
}

impl<R: RepositoriesModuleExt> TodoCreateUseCase<R> {
    pub async fn create_todo(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }
}

impl<R: RepositoriesModuleExt> TodoCreateTryUseCase<R> {
        // pub async fn create_todo(&self, title: String, description: String) -> anyhow::Result<Vec<Todo>> {
        //     // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        //     // Java だと return null; とかで Mock の開発ができるが、それをやりたい
        //     Ok((null()));
        // }
}