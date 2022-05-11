use axum::{routing::get, Router};
use rust_web_handson_presentation::{
    bootstrap::{init_app, startup},
    controller::hc::hc,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let app = Router::new().route("/hc", get(hc));
    let _ = startup(app).await;

    Ok(())
}
