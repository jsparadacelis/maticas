use std::fmt;
use std::future::Future;

use super::metric::Metric;

#[derive(Debug)]
pub struct MetricsRepositoryError(pub String);

impl fmt::Display for MetricsRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "metrics repository error: {}", self.0)
    }
}

impl std::error::Error for MetricsRepositoryError {}

pub trait MetricsRepository {
    fn save(&self, metric: Metric) -> impl Future<Output = Result<(), MetricsRepositoryError>> + Send;
}
