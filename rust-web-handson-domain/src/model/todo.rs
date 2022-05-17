use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(new, Debug)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
}
