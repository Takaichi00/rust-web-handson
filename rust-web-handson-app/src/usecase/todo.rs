use std::sync::Arc;

use derive_new::new;
use mockall::automock;
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

#[automock]
impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    pub async fn get_list(&self) -> anyhow::Result<Vec<Todo>> {
        self.repositories.todo_repository().get_all().await
    }
}

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
    // TODO presentation → application の DTO は最初のうちは作らずに、presentation そうで Domain モデルを組み立ててしまう方針とする
    pub async fn create_todo(&self, new_todo: NewTodo) -> anyhow::Result<()> {
        // await 忘れがち part2...
        // NewTodo を presentation で作成するかどうか?
        // let result = self.repositories.todo_repository().insert(NewTodo::new(title, description)).await;

        // self.repositories.todo_repository().insert(NewTodo::new(title, description)).await.map(|op| Ok(op))? → これは↓と同義。map okを wrap してそれを ? で展開しているので冗長
        self.repositories.todo_repository().insert(new_todo).await
        // ? が () or Err(e) を返す match 文をかいてくれる
        // ラムダ式 (|op|) で Ok(()) としてあげることで置き換えることができる
        // map → Ok のときにこういう内部処理をしてくださいねということをしている
        // map_err() → エラーのときにこういうことをしてくださいを定義できる
        // いちいち match でやるとコードの長さが多くなるので、 map, map_err で記載することが多い

        // TODO result で match しないと関数が実行されないのはどういう仕様?? → 実行されます。
        // self.repositories.todo_repository().insert(NewTodo::new(title, description)).await
        // match result {
        //     Ok(_result) => {
        //         Ok(())
        //     }
        //     Err(e) => {
        //         Err(e)
        //     }
        // }


    }
}

impl<R: RepositoriesModuleExt> TodoUseCase<R> {
        // pub async fn create_todo_try(&self, title: String, description: String) -> anyhow::Result<Vec<Todo>> {
        //     // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        //     // Java だと return null; とかで Mock の開発ができるが、それをやりたい
        //     Ok((null()));
        // }
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::Local;
    use rust_web_handson_domain::repository::todo::MockTodoRepository;
    use rust_web_handson_infra::modules::MockRepositoriesModuleExt;

    #[tokio::test]
    async fn test_get_list() -> () {
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

        let todo_usecase = TodoUseCase::new(Arc::new(mock_repositories));
        let result = todo_usecase.get_list().await;

        let expect = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];

        assert_eq!(result.unwrap(), expect);
    }
}