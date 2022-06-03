use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use rust_web_handson_app::modules::UseCaseModules;

use rust_web_handson_app::{modules::UseCaseModulesExt, usecase::todo::TodoUseCase};
use rust_web_handson_domain::model::todo::NewTodo;

use crate::model::{
    todo::TodoJson, todo_create_request::TodoCreateRequestJson,
    todo_create_response::TodoCreateResponseJson,
};

// insert をするときは route を追加 → 対応するメソッドを追加する
pub fn router() -> Router {
    return Router::new()
        .route("/", get(get_all::<UseCaseModules>))
        .route("/", post(create))
        .route("/try", post(create_try));
}

/**
 * Todo を取得する.
 */
pub async fn get_all<U: UseCaseModulesExt>(
    Extension(modules): Extension<Arc<U>>,
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

/**
 * Todo を作成する (Hands On ver)
 */
pub async fn create(
    Json(request_json): Json<TodoCreateRequestJson>,
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = modules
        .todo_usecase()
        .create_todo(NewTodo::from(request_json))
        .await;

    match result {
        Ok(_result) => {
            let mut headers = HeaderMap::new();
            headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());

            return Ok((StatusCode::CREATED, headers));
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

/**
 * Todo を作成する (Try バージョン)
 */
pub async fn create_try(
    Json(request_json): Json<TodoCreateRequestJson>,
    // Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    if true {
        let mock_response: TodoCreateResponseJson = TodoCreateResponseJson::new(
            1,
            "hogehoge".to_string(),
            "fugafuga".to_string(),
            "2022-01-01 01:00:00".to_string(),
        );
        let body: Json<TodoCreateResponseJson> = Json(mock_response);

        let mut headers = HeaderMap::new();
        headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());

        // TODO 下記は IntoResponse 型? どのように初期化しているか?
        return Ok((StatusCode::CREATED, headers, body));
    }

    return Err(StatusCode::INTERNAL_SERVER_ERROR);
}

#[cfg(test)]
mod tests {

    use chrono::Local;
    use rust_web_handson_app::{modules::MockUseCaseModulesExt, usecase::todo::MockTodoUseCase};
    use rust_web_handson_domain::model::todo::Todo;

    use super::*;

    #[tokio::test]
    async fn get_allが正常に成功した場合はStatusCode_CREATEDが取得できる() {
        // setup
        let mut mock_usecase_module = MockUseCaseModulesExt::new();
        let mut mock_todo_usecase = MockTodoUseCase::new();

        let now = Local::now();

        let select = vec![Todo::new(
            1,
            "hoge".to_string(),
            "fuga".to_string(),
            now.clone(),
            now.clone(),
            Some(now.clone()),
        )];

        let expect_result: anyhow::Result<Vec<Todo>> = anyhow::Ok(select.to_vec());

        mock_todo_usecase
            .expect_get_list()
            .return_once(|| expect_result);

        mock_usecase_module
            .expect_todo_usecase()
            .once()
            .return_const(mock_todo_usecase);

        // execute
        let actual = get_all(Extension(Arc::new(mock_usecase_module)))
            .await
            .unwrap();

        // assert
        let expected_status = StatusCode::OK;
        let actual_status_code = actual.into_response().status().clone();

        // TODO Response Body をアサーションする
        // fn expect_response(select: Vec<Todo>) -> impl IntoResponse {
        //     let expected_json: Json<Vec<TodoJson>> =
        //         Json(select.into_iter().map(|t| TodoJson::from(t)).collect());
        //     (StatusCode::OK, expected_json)
        // }

        assert_eq!(expected_status, actual_status_code);

        // assert_eq!(
        //     actual.into_response(),
        //     expect_response(select).into_response()
        // );

        // let actual_json = actual.into_response().body();
        // assert_eq!(actual_status_code, expected_status);
        // assert_eq!(actual_json, expected_json);
    }
}
