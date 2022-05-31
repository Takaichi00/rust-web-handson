use rust_web_handson_domain::model::todo::NewTodo;
use serde::{Deserialize};

// フィールドは public にしてもいいかな...?　どっちでもいい。
// ボイラープレートが増えるので、public にすることが多い
// ボイラープレート... プログラム上必要なんだけど冗長なコード
#[derive(Deserialize, Debug)]
pub struct TodoCreateRequestJson {
    pub title: String,
    pub description: String
}

impl TodoCreateRequestJson {
    pub fn new (title: String, description: String) -> Self {

        Self {
            title,
            description
        }
    }

    // この時点で参照を渡している
    pub fn getTitle(&self) -> &String {
        &self.title
    }

    pub fn getDescription(&self) -> &String {
        &self.description
    }
}

// TODO
// from trait を実装する
impl From<TodoCreateRequestJson> for NewTodo {
    // from メソッドを実装
    fn from(todo_create_requet_json: TodoCreateRequestJson) -> Self {
        NewTodo::new(
            todo_create_requet_json.title,
            todo_create_requet_json.description,
        )
    }
}