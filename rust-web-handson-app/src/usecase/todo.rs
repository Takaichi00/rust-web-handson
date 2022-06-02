use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use rust_web_handson_domain::{model::todo::{Todo, NewTodo}, repository::{todo::TodoRepository, RepositoriesModuleExt}};

use super::UseCaseImpl;

#[automock]
#[async_trait]
pub trait TodoUseCase {
    async fn get_list(&self) -> anyhow::Result<Vec<Todo>>;
    async fn create_todo(&self, new_todo: NewTodo) -> anyhow::Result<()>;
}

// 動的型付をする際に、安全に並列処理が実行できるようにする
#[async_trait]
impl<R: RepositoriesModuleExt + Sync + Send> TodoUseCase for UseCaseImpl<Todo, R> {
    async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }

    async fn create_todo(&self, new_todo: NewTodo) -> anyhow::Result<()> {
        self.repositories.todo_repository().insert(new_todo).await
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::Local;
    use rust_web_handson_domain::repository::{todo::MockTodoRepository, MockRepositoriesModuleExt};

    #[tokio::test]
    async fn test_get_list() -> () {
        
        // setup
        let mut mock_repositories = MockRepositoriesModuleExt::new();
        let mut mock_todo_repo = MockTodoRepository::new();

        let now = Local::now();

        let select = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];

        let expect_result: anyhow::Result<Vec<Todo>> = anyhow::Ok(select.to_vec());

        mock_todo_repo
            .expect_get_all()
            .return_once(|| expect_result);

        mock_repositories
            .expect_todo_repository()
            .once()
            .return_const(mock_todo_repo);

        let todo_usecase = UseCaseImpl::new(Arc::new(mock_repositories));

        // execte
        let result = todo_usecase.get_list().await;

        let expect = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];
        
        // assert
        assert_eq!(result.unwrap(), expect);
    }
}