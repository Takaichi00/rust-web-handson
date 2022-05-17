use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use rust_web_handson_app::modules::{UseCaseModules, UseCaseModulesExt};

use crate::model::todo::TodoJson;

pub fn router() -> Router {
    Router::new().route("/", get(get_all))
}

pub async fn get_all(
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo_list = modules.todo_usecase().get_list().await;
    match todo_list {
        Ok(tl) => {
            let body: Json<Vec<TodoJson>> =
                Json(tl.into_iter().map(|t| TodoJson::from(t)).collect());
            Ok((StatusCode::OK, body))
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
