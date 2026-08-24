use reqwest::Client;
use serde::Serialize;

use crate::maticas::domain::metric::Metric;
use crate::maticas::domain::metrics_repository::{MetricsRepository, MetricsRepositoryError};

#[derive(Serialize)]
struct MetricRow {
    temperature_c: f64,
}

pub struct SupabaseMetricsRepository {
    http: Client,
    base_url: String,
    api_key: String,
}

impl SupabaseMetricsRepository {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
            api_key,
        }
    }
}

impl MetricsRepository for SupabaseMetricsRepository {
    async fn save(&self, metric: Metric) -> Result<(), MetricsRepositoryError> {
        let url = format!("{}/rest/v1/metrics", self.base_url);
        let row = MetricRow {
            temperature_c: metric.temperature_c,
        };

        let response = self
            .http
            .post(url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Prefer", "return=minimal")
            .json(&row)
            .send()
            .await
            .map_err(|err| MetricsRepositoryError(err.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(MetricsRepositoryError(format!(
                "supabase respondió {status}: {body}"
            )));
        }

        Ok(())
    }
}
