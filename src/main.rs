use axum::{routing::get, Router};
use dotenv::dotenv;

mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a single route
    let app = Router::new()
        .route("/get", get(get_pubkey))
        .route("/getBalance", get(|| get_balance(true)));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_pubkey() -> String {
    util::basic_util::get_pubkey("MY_PUB_KEY").to_string()
}

async fn get_balance(is_allow: bool) {
    println!("{}", is_allow);
}
