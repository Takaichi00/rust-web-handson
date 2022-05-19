use async_trait::async_trait;
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
            // Ok(a) => a, Err(e) => Err(e) と同義
            // await? はエラーだったら中身を取り出し、エラーじゃかなったらエラーを出すという意味と同じ
        todo_list.into_iter().map(|t| Todo::try_from(t)).collect()
        
        // iterator の方に変換
        // 色々やった処理を collect で元の処理に戻す
        // collect にしないと、無限に collect のまま
        // into_iter → 実体が渡る, iter → 借用, iter_mut → mutable が渡る
        // 実行するだけでいいなら借用など→難関
        // map とかやるときに、脳死で todo_list.iter().map → t でやるとき、「t に方があればいいのにとなったら、into_iter を使う」
        // (|t|) は t=> と同じ
        // try_from が result を返してくる
    }

    async fn insert(&self, source: NewTodo) -> anyhow::Result<()> {
        let pool = self.pool.0.clone();
        sqlx::query("insert into todo (name, description) values (?, ?)")
            .bind(source.title)
            .bind(source.description)
            .execute(&*pool)
            .await?;
        Ok(())
    }
}