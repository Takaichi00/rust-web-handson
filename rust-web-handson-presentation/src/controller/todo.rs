use std::{sync::Arc};

use axum::{http::{StatusCode, HeaderMap}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};

use mockall_double::*;

#[double]
use rust_web_handson_app::modules::UseCaseModules;
use rust_web_handson_domain::model::todo::NewTodo;

use crate::model::{todo::TodoJson, todo_create_response::TodoCreateResponseJson, todo_create_request::TodoCreateRequestJson};

// insert をするときは route を追加 → 対応するメソッドを追加する
pub fn router() -> Router {
    return Router::new().route("/", get(get_all))
                        .route("/", post(create))
                        .route("/try", post(create_try));
}

/**
 * Todo を取得する.
 */
pub async fn get_all(
    // Extension → DI するためのコンテナになる
    // acsim が定義された Extension を Controller に渡している
    // ただその設定は必要である
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    // TODO ここでは Todo のドメインモデルを受け取っているが、pressentation 層からドメインモデルを直接いじれてしまうのは好ましくないので application 層から presentation にわたすための DTO を作ったりする。
    // L tokio だとそうすると冗長になってしまうか?
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

/**
 * Todo を作成する (Hands On ver)
 */
pub async fn create(
    Json(request_json): Json<TodoCreateRequestJson>,
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {

    // await 忘れがち...
    // TODO clone() でいいのか? それとも &String で受けたほうがよい?
    // getTitle() の中で .clone を実行するのがベター。参照を返すというのはそんなにしない。
    // int, string などもともとある型が大半。
    let result = modules.todo_usecase()
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
        let mock_response: TodoCreateResponseJson = TodoCreateResponseJson::new(1, "hogehoge".to_string(), "fugafuga".to_string(), "2022-01-01 01:00:00".to_string());
        let body: Json<TodoCreateResponseJson> = Json(mock_response);
        
        let mut headers = HeaderMap::new();
        headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());

        // TODO 下記は IntoResponse 型? どのように初期化しているか?
        return Ok((StatusCode::CREATED, headers, body));
    }

    return Err((StatusCode::INTERNAL_SERVER_ERROR));
}


#[cfg(test)]
mod tests {
    
    use std::fmt::Debug;

    use axum::body::HttpBody;
    use rust_web_handson_app::modules::UseCaseModules;

    use crate::model::todo_create_response::TodoCreateResponseJson;
    
    use std::collections::HashMap;

    use mockall_double::*;

    // #[double]
    // use rust_web_handson_app::modules::MockUseCaseModules;


    #[tokio::test]
    async fn createが正常に成功した場合はStatusCode_CREATEDが取得できる() {

    }

    #[tokio::test]
    async fn createが正常に成功した場合はStatusCode_CREATEDが取得できる_old() {

        // setup 
        // TODO UseCaseModules を Mock にする
        // TODO test 時に test 用の DI コンテナを作成する方法は??

        // #[derive(UseCaseModules)]
        // struct MockUseCase {
            
        // }

        // let mock_usecase = 

        // let expected_status = StatusCode::CREATED;
        
        // execute
        // let actual = create_try(Json(TodoCreateRequestJson::new("test-title".to_string(), "test-description".to_string())), mock_usecase)
        //                                         .await
        //                                         .unwrap();


        // let mut headers = HeaderMap::new();
        // headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());
        // let mock_response: TodoCreateResponseJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string());
        // let body: Json<TodoCreateResponseJson> = Json(mock_response);

        // // let expected: IntoResponse = impl IntoResponse { 
        // //     StatusCode::CREATED, headers, body
        // // };

        // // assert_eq!(s, expected);

        // // assert
        // let actual_status_code = actual.into_response().status().clone();
        // assert_eq!(actual_status_code, expected_status);

        
        // TODO
        // Response Body をどう取得するか?
        // Header をどのように取得するか?
        // そもそも IntoResponse 型でアサーションできないか? → これをやってしまうと test が実装を知ってしまっているのでやりたくない??
        // L それとも test 内で 201, "Location: 'http://example'", "{ "result": true}" のように設定できるか?

        // let actual_json = actual.into_response().into_body();
        // print!("{}", actualJson);

    }

}