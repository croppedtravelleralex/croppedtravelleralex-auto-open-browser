use async_trait::async_trait;

use crate::{error::AppError, model::task::TaskRecord};

use super::models::BrowserRunResult;

#[async_trait]
pub trait BrowserAdapter: Send + Sync {
    async fn healthcheck(&self) -> Result<(), AppError>;
    async fn run_task(&self, task: &TaskRecord) -> Result<BrowserRunResult, AppError>;
}
