use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

#[derive(Clone)]
pub struct AppRegistory {
    health_check_repogitory: Arc<dyn HealthCheckRepository>,
}

impl AppRegistory {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repogitory = Arc::new(HealthCheckRepositoryImpl::new(pool));
        Self {
            health_check_repogitory,
        }
    }

    pub fn health_check_repogitory(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repogitory.clone()
    }
}
