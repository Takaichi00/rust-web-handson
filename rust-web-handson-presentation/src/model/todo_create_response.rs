// use rust_web_handson_domain::model::todo::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub struct TodoCreateResponseJson {
    id: i64,
    title: String,
    description: String,
    created_at: String,
}

impl TodoCreateResponseJson {
    pub fn new (id: i64, title: String, description: String, created_at: String) -> Self {

        Self {
            id,
            title,
            description,
            created_at
        }
    }
}

// impl From<Todo> for TodoCreateResponseJson {
//     fn from(t: Todo) -> Self {
//         TodoJson {
//             id: t.id,
//             title: t.title,
//             description: t.description,
//             created_at: t.created_at.to_string(),
//         }
//     }
// }
