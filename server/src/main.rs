use std::sync::Arc;

use crate::maticas::infrastructure::api::send_metrics_endpoint;
use crate::maticas::infrastructure::repository::supabase_metrics_repository::SupabaseMetricsRepository;
use axum::routing::post;
use axum::Router;

mod maticas;

#[tokio::main]
async fn main() {
    run_http_server().await;
}

async fn run_http_server() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let repository = build_metrics_repository();
    let router = create_router(repository);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

fn build_metrics_repository() -> Arc<SupabaseMetricsRepository> {
    let base_url = std::env::var("SUPABASE_URL").expect("falta SUPABASE_URL");
    let api_key =
        std::env::var("SUPABASE_SERVICE_ROLE_KEY").expect("falta SUPABASE_SERVICE_ROLE_KEY");
    Arc::new(SupabaseMetricsRepository::new(base_url, api_key))
}

async fn handle_get() -> &'static str {
    "ok"
}

pub fn create_router(repository: Arc<SupabaseMetricsRepository>) -> Router {
    Router::new()
        .route(
            "/metrics",
            post(send_metrics_endpoint::handle::<SupabaseMetricsRepository>).get(handle_get),
        )
        .with_state(repository)
}

