use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::maticas::domain::metric::Metric;
use crate::maticas::domain::metrics_repository::MetricsRepository;

#[derive(Deserialize)]
pub(crate) struct MetricPayload {
    temperature_c: f64,
}

pub async fn handle<R: MetricsRepository>(
    State(repository): State<Arc<R>>,
    Json(payload): Json<MetricPayload>,
) -> (StatusCode, &'static str) {
    let metric = Metric {
        temperature_c: payload.temperature_c,
    };

    match repository.save(metric).await {
        Ok(()) => (StatusCode::OK, "ok"),
        Err(err) => {
            eprintln!("error guardando metric: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "error guardando la métrica")
        }
    }
}
