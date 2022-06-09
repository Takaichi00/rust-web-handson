use crate::model::todo::{NewTodo, Todo};
use async_trait::async_trait;
use mockall::automock;
#[automock]
#[async_trait] // trait に async の function は定義できないので、ライブラリを使う
pub trait TodoRepository {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn insert(&self, source: NewTodo) -> anyhow::Result<()>;
}
