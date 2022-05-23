use std::{sync::Arc, ptr::null};

use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use rust_web_handson_app::modules::{UseCaseModules, UseCaseModulesExt};

use crate::model::todo::TodoJson;

// insert をするときは route を追加 → 対応するメソッドを追加する
pub fn router() -> Router {
    return Router::new().route("/", get(get_all))
                 .route("/", post(create));
}

pub async fn get_all(
    // Extension → DI するためのコンテナになる
    // acsim が定義された Extension を Controller に渡している
    // ただその設定は必要である
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo_list = modules.todo_usecase().get_list().await;
    match todo_list {
        Ok(tl) => {
            // ここだけ型定義している
            // collect したとき、アサーションがうまくできない。もらっているものがわからない
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

// pub async fn create() -> Result<impl IntoResponse, StatusCode> {
//     Ok((StatusCode::CREATED));
// }

pub async fn create() -> StatusCode {
    return StatusCode::CREATED;
}