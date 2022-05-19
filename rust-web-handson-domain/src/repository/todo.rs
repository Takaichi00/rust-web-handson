use async_trait::async_trait;
use crate::model::todo::{NewTodo, Todo};
// trait に async の function ははやせないので、ライブラリを使う
#[async_trait]
pub trait TodoRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<()>;
}