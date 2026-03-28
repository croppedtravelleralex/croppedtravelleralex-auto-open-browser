use std::sync::Arc;

use crate::{
    db::init::DbPool,
    queue::memory::MemoryTaskQueue,
    runner::TaskRunner,
};

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub queue: MemoryTaskQueue,
    pub api_key: Option<String>,
    pub runner: Arc<dyn TaskRunner>,
    pub worker_count: usize,
}
