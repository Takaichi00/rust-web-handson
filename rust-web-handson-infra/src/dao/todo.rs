
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