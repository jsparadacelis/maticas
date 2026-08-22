use axum::body::Bytes;

const ENDPOINT: &str = "/metrics";

pub async fn handle(body: Bytes) -> &'static str {
    println!("received: {}", String::from_utf8_lossy(&body));
    "ok"
}
