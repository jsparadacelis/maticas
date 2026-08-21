use axum::Router;
use axum::routing::{post, get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handle_post)).route("/detail", get(handle_get));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handle_post(body: String) -> &'static str {
    println!("received: {body}");
    "ok"
}

async fn handle_get() -> &'static str {
    "ok"
}
