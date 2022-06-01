
use chrono::{DateTime, Local};
use derive_new::new;

// Debug → println をするために継承している
// new → derive_new というクラスになる
// Lombok みたいなもの
#[derive(new, Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

// new したときに id なんて入れられないよ〜というために↓を作る
#[derive(new, Debug)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
}
