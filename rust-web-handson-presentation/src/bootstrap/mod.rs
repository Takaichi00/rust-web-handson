use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router};
use dotenv::dotenv;
use rust_web_handson_app::modules::UseCaseModules;
use rust_web_handson_infra::client::mysql::Rds;

pub async fn startup(router: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    let rds = Rds::new().await;

    // UseCaseModules に rds のインスタンスを渡してあげないと行けない
    let modules = Arc::new(UseCaseModules::new(rds).await);

    // 共通の値をシングルトンで渡してくださいという処理
    // 例えば、AuthenticationLayer 、Controlelr にわたす前の Filter のような処理
    // Filter の処理で Controller に
    let app = router.layer(Extension(modules));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}
