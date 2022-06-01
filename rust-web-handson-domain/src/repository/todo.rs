use async_trait::async_trait;
use crate::model::todo::{NewTodo, Todo};
use mockall::automock;
#[automock]
#[async_trait] // trait に async の function ははやせないので、ライブラリを使う
pub trait TodoRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<()>;
}
