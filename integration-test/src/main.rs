// main 関数がないと怒られてしまう
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn sampleEtoETestAsync() -> Result<(), Box<dyn std::error::Error>> {

        // TODO json を作って頑張ってアサーションする? それとも text → json のパースを頑張るか
        let client = reqwest::Client::new();
        let res = client.post("http://127.0.0.1:8080/todo")
            .body("{ \"title\": \"hogehgoe\", \"description\": \"fugafuga\" }")
            .header("Content-Type", "application/json")
            .send()
            .await?;
            // TODO i64 → String の キャストは勝手にやってくれなさそう...
            // .json::<HashMap<String, String>>()
            // .await?;

        assert_eq!(res.status(), 201);

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