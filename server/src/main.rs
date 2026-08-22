use crate::maticas::infrastructure::api::send_metrics_endpoint;
use axum::routing::post;
use axum::Router;

mod maticas;

#[tokio::main]
async fn main() {
    run_http_server().await;
}

async fn run_http_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let router = create_router();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

async fn handle_get() -> &'static str {
    "ok"
}

pub fn create_router() -> Router {
    Router::new().route(
        "/metrics",
        post(send_metrics_endpoint::handle).get(handle_get),
    )
}

