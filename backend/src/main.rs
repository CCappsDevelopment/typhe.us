use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build application with a route
    let app = Router::new().route("/", get(root));

    // bind the TCP listener to 127.0.0.1:3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("Listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World! Rust backend running on Axum 0.8"
}

