use std::{sync::Arc, ptr::null};

use derive_new::new;
use rust_web_handson_domain::{model::todo::{Todo, NewTodo}, repository::todo::TodoRepository};
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


/*
 * ↓ Impl
 */


impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }
}

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub async fn create_todo(&self, title: String, description: String) -> anyhow::Result<()> {
        // await 忘れがち part2...
        let result = self.repositories.todo_repository().insert(NewTodo::new(title, description)).await;

        // TODO result で match しないと関数が実行されないのはどういう仕様??
        // self.repositories.todo_repository().insert(NewTodo::new(title, description)).await; だけでは処理が実行されない
        match result {
            Ok(_result) => {
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
        // pub async fn create_todo(&self, title: String, description: String) -> anyhow::Result<Vec<Todo>> {
        //     // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        //     // Java だと return null; とかで Mock の開発ができるが、それをやりたい
        //     Ok((null()));
        // }
}