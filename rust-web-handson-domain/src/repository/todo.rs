use async_trait::async_trait;

use crate::model::todo::{NewTodo, Todo};

#[async_trait]
pub trait TodoRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<()>;
}
