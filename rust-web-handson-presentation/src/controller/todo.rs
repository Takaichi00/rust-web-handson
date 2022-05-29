use std::{sync::Arc, ptr::null};

use axum::{http::{StatusCode, HeaderMap}, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use rust_web_handson_app::modules::{UseCaseModules, UseCaseModulesExt};

use crate::model::{todo::TodoJson, todo_create_response::TodoCreateResponseJson, todo_create_request::TodoCreateRequestJson};

// insert をするときは route を追加 → 対応するメソッドを追加する
pub fn router() -> Router {
    return Router::new().route("/", get(get_all))
                        .route("/", post(create))
                        .route("/try", post(create_try));
}

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

pub async fn create(
    Json(request_json): Json<TodoCreateRequestJson>,
    Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {

    // await 忘れがち...
    // TODO clone() でいいのか? それとも &String で受けたほうがよい?
    let result = modules.todo_usecase().create_todo(request_json.getTitle().clone(), request_json.getDescription().clone()).await;

    match result {
        Ok(_result) => {
            let mut headers = HeaderMap::new();
            headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());

            return Ok((StatusCode::CREATED, headers));
        }
        Err(e) => {
            tracing::error!("Error : {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR));       
        }
    }
}




pub async fn create_try(
    // Extension(modules): Extension<Arc<UseCaseModules>>,
) -> Result<impl IntoResponse, StatusCode> {
    if true {
        let mockResponse: TodoCreateResponseJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string());
        let body: Json<TodoCreateResponseJson> = Json(mockResponse);
        
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

    use crate::model::todo_create_response::TodoCreateResponseJson;
    
    use mockall::{automock, mock, predicate::*};
    
    use std::collections::HashMap;

    use super::*;

    // #[cfg_attr(test, automock)]
    // impl UseCaseModules {

    // }

    #[tokio::test]
    async fn createが正常に成功した場合はStatusCode_CREATEDが取得できる() {

        // TODO UseCaseModules を Mock にする
        let expectedStatus = StatusCode::CREATED;
        // let expectedJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string());

        // test 時に test 用の DI コンテナを作成する方法は??
        let actual = create_try().await.ok();
    
        match actual {
            Some(s) => {

                let mut headers = HeaderMap::new();
                headers.insert("Location", "http://localhost:8080/todo/1".parse().unwrap());
                let mockResponse: TodoCreateResponseJson = TodoCreateResponseJson::new(1, "title".to_string(), "description".to_string(), "2022-01-01 01:00:00".to_string());
                let body: Json<TodoCreateResponseJson> = Json(mockResponse);

                // let expected: IntoResponse = impl IntoResponse { 
                //     StatusCode::CREATED, headers, body
                // };

                // assert_eq!(s, expected);

                let actualStatusCode = s.into_response().status();
                assert_eq!(actualStatusCode, expectedStatus);

                
                // TODO
                // Response Body をどう取得するか?
                // Header をどのように取得するか?
                // そもそも IntoResponse 型でアサーションできないか? → これをやってしまうと test が実装を知ってしまっているのでやりたくない??
                // L それとも test 内で 201, "Location: 'http://example'", "{ "result": true}" のように設定できるか?

                // let actualJson = s.into_response().into_body().boxed().
                // print!("{}", actualJson);
            },
            None => unreachable!(),
        }
    }

    // TODO 以下 EtoE テスト、別ディレクトリに分ける方法はどのようにするか?
    #[tokio::test]
    async fn sampleEtoETestAsync() -> Result<(), Box<dyn std::error::Error>> {

        // TODO json を作って頑張ってアサーションする? それとも text → json のパースを頑張るか
        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:8080/todo")
            .body("{}")
            .send()
            .await?
            // TODO i64 → String の キャストは勝手にやってくれなさそう...
            .json::<HashMap<String, String>>()
            .await?;

        assert_eq!("", "");

        // let resp = reqwest::get("https://httpbin.org/ip")
        // .await?
        // .json::<HashMap<String, String>>()
        // .await?;

        println!("{:#?}", res);
        Ok(())
    }

    #[test]
    fn sampleEtoETestBlocking() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get("https://httpbin.org/get")?
        // .json::<String>()?;
        .text()?;
        println!("{:#?}", resp);
        // assert_eq!(resp, "{\n  \"args\": {}, \n  \"headers\": {\n    \"Accept\": \"*/*\", \n    \"Host\": \"httpbin.org\", \n    \"X-Amzn-Trace-Id\": \"Root=1-628edd96-3ab7f59e2f687d615101b21b\"\n  }, \n  \"origin\": \"111.87.41.99\", \n  \"url\": \"https://httpbin.org/get\"\n}\n");
        Ok(())
    }

}