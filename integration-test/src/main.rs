// main 関数がないと怒られてしまう
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rust_web_handson_presentation::model::todo_create_response::TodoCreateResponseJson;

    #[tokio::test]
    async fn _201_todoを正常に作成することができる() -> Result<(), Box<dyn std::error::Error>> {

        // TODO json を作って頑張ってアサーションする? それとも text → json のパースを頑張るか
        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:8080/todo/try")
            .body("{ \"title\": \"hogehgoe\", \"description\": \"fugafuga\" }")
            .header("Content-Type", "application/json")
            .send()
            .await?;
            // 以下、i64 → String の キャストは勝手にやってくれなさそう
            // .json::<HashMap<String, String>>()

        let response_json = res.json::<TodoCreateResponseJson>().await?;
        assert_eq!(response_json.title, "hogehoge");
        assert_eq!(response_json.description, "fugafuga");
        // res.status();
        // assert_eq!(res.status().clone(), 201);

        // let resp = reqwest::get("https://httpbin.org/ip")
        // .await?
        // .json::<HashMap<String, String>>()
        // .await?;

        // println!("{:#?}", res);
        Ok(())
    }

    /**
     * Blocking のテスト実行↓
     */
    #[test]
    fn sample_e2e_test_blocking() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get("https://httpbin.org/get")?
        // .json::<String>()?;
        .text()?;
        println!("{:#?}", resp);
        Ok(())
    }

}