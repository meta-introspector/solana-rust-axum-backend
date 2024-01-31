use axum::{http::Method, routing::get, Router};
use dotenv::dotenv;
use service::solana_service;
use tower_http::cors::{Any, CorsLayer};

mod util;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/get", get(get_pubkey))
        .route("/getBalance", get(get_balance))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_pubkey() -> String {
    util::basic_util::get_pubkey().to_string()
}

async fn get_balance() -> String {
    solana_service::get_balance().to_string()
}
