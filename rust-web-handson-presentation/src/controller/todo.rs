use std::{sync::Arc, ptr::null};

use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use rust_web_handson_app::modules::{UseCaseModules, UseCaseModulesExt};

use crate::model::{todo::TodoJson, todo_create_response::TodoCreateResponseJson};

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

pub async fn create() -> Result<impl IntoResponse, StatusCode> {
    if true {
        let mockResponse: TodoCreateResponseJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string()).await;
        let body: Json<TodoCreateResponseJson> = Json(mockResponse);
        return Ok((StatusCode::CREATED, body));
    }

    return Err((StatusCode::INTERNAL_SERVER_ERROR));
}

#[cfg(test)]
mod tests {
    
    use std::fmt::Debug;

    use axum::body::HttpBody;

    use crate::model::todo_create_response::TodoCreateResponseJson;

    use super::*;

    #[tokio::test]
    async fn createが正常に成功した場合はStatusCode_CREATEDが取得できる() {
        let expectedStatus = StatusCode::CREATED;
        // let expectedJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string());
        let actual = create().await.ok();
    
        match actual {
            Some(s) => {
                let actualStatusCode = s.into_response().status();
                assert_eq!(actualStatusCode, expectedStatus);
                
                // Response Body をどう取得するか?

                // let actualJson = s.into_response().into_body().boxed().
                // print!("{}", actualJson);
            },
            None => unreachable!(),
        }

        // assert_eq!(actual, expected);
        // assert_eq!(create(), StatusCode::CREATED);
    }
}