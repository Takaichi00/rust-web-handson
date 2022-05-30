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
    // TODO presentation → application の DTO は最初のうちは作らずに、presentation そうで Domain モデルを組み立ててしまう方針とする
    pub async fn create_todo(&self, title: String, description: String) -> anyhow::Result<()> {
        // await 忘れがち part2...
        // NewTodo を presentation で作成するかどうか?
        // let result = self.repositories.todo_repository().insert(NewTodo::new(title, description)).await;

        // self.repositories.todo_repository().insert(NewTodo::new(title, description)).await.map(|op| Ok(op))? → これは↓と同義。map okを wrap してそれを ? で展開しているので冗長
        self.repositories.todo_repository().insert(NewTodo::new(title, description)).await
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
        // pub async fn create_todo(&self, title: String, description: String) -> anyhow::Result<Vec<Todo>> {
        //     // TODO とりあえず空実装したいが、どうしたらいいのかわからない
        //     // Java だと return null; とかで Mock の開発ができるが、それをやりたい
        //     Ok((null()));
        // }
}