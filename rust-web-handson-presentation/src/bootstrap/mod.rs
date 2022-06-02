use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router};
use dotenv::dotenv;
use rust_web_handson_app::modules::UseCaseModules;
use rust_web_handson_infra::client::mysql::Rds;

pub async fn startup(router: Router) {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    tracing::info!("Server listening on {}", addr);

    // UseCaseModules に rds のインスタンスを渡してあげないと行けない
    // TODO ここで rds の実装を渡すのはいいのだが、UseCaseModules::new で受ける方を trait or struct にすることはできないか? application 層の UseCaseModules が Infra 層に依存してしまっている。できるなら Domain 層の TodoRepository の型で受けたい
    // TODO それかここで UseCaseModules::new(rds) の処理を実施することで、UseCaseModules は Infra のことを知らなくてすむ
    let modules = Arc::new(UseCaseModules::new().await);

    // 共通の値をシングルトンで渡してくださいという処理
    // 例えば、AuthenticationLayer 、Controlelr にわたす前の Filter のような処理
    // Filter の処理で Controller に DI 対象のコンポーネントを渡している
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
