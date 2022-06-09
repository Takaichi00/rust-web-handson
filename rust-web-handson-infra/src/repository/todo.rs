use async_trait::async_trait;
use chrono::{Local, TimeZone};
use rust_web_handson_domain::{
    model::todo::{NewTodo, Todo},
    repository::todo::TodoRepository,
};
use sqlx::query_as;

use crate::dao::todo::TodoTable;

use super::RdsRepositoryImpl;

#[async_trait]
impl TodoRepository for RdsRepositoryImpl<Todo> {
    async fn get_all(&self) -> anyhow::Result<Vec<Todo>> {
        let pool = self.pool.0.clone();
        let todo_list = query_as::<_, TodoTable>("select * from todo")
            .fetch_all(&*pool)
            .await?;
        todo_list.into_iter().map(|t| Todo::try_from(t)).collect()
    }

    async fn get(&self, id: i64) -> anyhow::Result<Todo> {
        let pool = self.pool.0.clone();
        let todo = query_as::<_, TodoTable>("select * from todo where id = ?")
            .bind(id)
            .fetch_one(&*pool)
            .await?;
        Todo::try_from(todo)
    }

    async fn insert(&self, source: NewTodo) -> anyhow::Result<i64> {
        let pool = self.pool.0.clone();
        let result = sqlx::query("insert into todo (title, description) values (?, ?)")
            .bind(source.title)
            .bind(source.description)
            .execute(&*pool)
            .await?;
        i64::try_from(result.last_insert_id()).map_err(anyhow::Error::msg)
    }

    async fn create_and_get_info(&self, source: NewTodo) -> anyhow::Result<Todo> {
        let id = self.insert(source).await?;
        self.get(id).await
    }
}

#[cfg(test)]
mod test {
    use rust_web_handson_domain::{
        model::todo::{NewTodo, Todo},
        repository::todo::TodoRepository,
    };

    use crate::{client::mysql::Rds, repository::RdsRepositoryImpl};

    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_all() -> () {
        dotenv().ok();
        let rds = Rds::new().await;
        let todo_repository: RdsRepositoryImpl<Todo> = RdsRepositoryImpl::new(rds);
        let result_list = todo_repository.get_all().await.unwrap();
        println!("{:?}", result_list.get(0).unwrap());
    }

    #[tokio::test]
    async fn test_insert() -> () {
        dotenv().ok();
        let rds = Rds::new().await;
        let todo_repository: RdsRepositoryImpl<Todo> = RdsRepositoryImpl::new(rds);
        let new_todo = NewTodo::new(String::from("test title"), String::from("test description"));
        todo_repository.insert(new_todo).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_and_get_info() -> () {
        dotenv().ok();
        let rds = Rds::new().await;
        let todo_repository: RdsRepositoryImpl<Todo> = RdsRepositoryImpl::new(rds);
        let new_todo = NewTodo::new(String::from("test title"), String::from("test description"));
        let result = todo_repository.create_and_get_info(new_todo).await.unwrap();
        assert_eq!(result.title, "test title");
        assert_eq!(result.description, "test description");
    }
}
