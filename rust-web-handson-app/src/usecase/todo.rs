use std::sync::Arc;

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

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }
}
