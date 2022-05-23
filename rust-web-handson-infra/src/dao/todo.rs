use chrono::{DateTime, Local};
use rust_web_handson_domain::model::todo::Todo;
use sqlx::FromRow;
#[derive(FromRow)] // from ... 変換処理を司っている Rust の基本処理 を継承するための記述、マッピングしてくれる
pub struct TodoTable {
    id: i64, // bigint
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>, // chrono の Datetime → Local 時間 (動いているサーバー時間に合わせてパースする)
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
// TodoTable.from(Row) 


// Trait がデフォで用意されている
// From, TryFrom → 共通の Interface のようなもの
// ドメインモデルのメソッドとして用意している
impl TryFrom<TodoTable> for Todo {
    type Error = anyhow::Error;
    fn try_from(tt: TodoTable) -> Result<Self, Self::Error> {
        Ok(Todo::new(
            tt.id,
            tt.title,
            tt.description,
            tt.created_at,
            tt.updated_at,
            tt.deleted_at,
        ))
    }
}