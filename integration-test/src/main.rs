// main 関数がないと怒られてしまう
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {

    use reqwest::Response;
    use rust_web_handson_presentation::model::todo_create_response::TodoCreateResponseJson;
    use std::sync::Once;

    static INIT: Once = Once::new();
    // static INIT: Once = OnceCell::new(); → ライブリを入れる。こっちがモダン。lazy 、使われるまで評価しない (かも)

    pub fn initialize() {
        println!("Before Each");
        INIT.call_once(|| {
            println!("Before All");
        });
    }

    #[tokio::test]
    async fn _201_todoを正常に作成することができる() -> anyhow::Result<()> {
        initialize();
        println!("initialize end 1");
        let client = reqwest::Client::new();
        let res: Response = client
            .post("http://127.0.0.1:8080/todo/try")
            .body("{ \"title\": \"hogehgoe\", \"description\": \"fugafuga\" }")
            .header("Content-Type", "application/json")
            .send()
            .await?;
        // 以下、i64 → String の キャストは勝手にやってくれなさそう
        // .json::<HashMap<String, String>>()

        assert_eq!(res.status().as_u16(), 201);
        assert_eq!(res.headers().contains_key("Location"), true);
        assert_eq!(
            res.headers().get("Location").unwrap(),
            &"http://localhost:8080/todo/1"
        );

        // テストケースごとにテストデータを用意できれば id なども検証できる (やる必要があるかは考えどころ)
        let response_json = res.json::<TodoCreateResponseJson>().await?;
        assert_eq!(response_json.title, "hogehoge");
        assert_eq!(response_json.description, "fugafuga");
        Ok(())
    }

    #[tokio::test]
    async fn _201_todoを正常に作成することができる_2() -> anyhow::Result<()> {
        initialize();
        println!("initialize end 2");
        let client = reqwest::Client::new();
        let res: Response = client
            .post("http://127.0.0.1:8080/todo/try")
            .body("{ \"title\": \"hogehgoe\", \"description\": \"fugafuga\" }")
            .header("Content-Type", "application/json")
            .send()
            .await?;
        // 以下、i64 → String の キャストは勝手にやってくれなさそう
        // .json::<HashMap<String, String>>()

        assert_eq!(res.status().as_u16(), 201);
        assert_eq!(res.headers().contains_key("Location"), true);
        assert_eq!(
            res.headers().get("Location").unwrap(),
            &"http://localhost:8080/todo/1"
        );

        // テストケースごとにテストデータを用意できれば id なども検証できる (やる必要があるかは考えどころ)
        let response_json = res.json::<TodoCreateResponseJson>().await?;
        assert_eq!(response_json.title, "hogehoge");
        assert_eq!(response_json.description, "fugafuga");
        Ok(())
    }

    /**
     * Blocking のテスト実行例
     */
    #[test]
    fn sample_e2e_test_blocking() -> Result<(), Box<dyn std::error::Error>> {
        initialize();
        println!("initialize end 3");
        let resp = reqwest::blocking::get("https://httpbin.org/get")?
            // .json::<String>()?;
            .text()?;
        println!("{:#?}", resp);
        Ok(())
    }
}
