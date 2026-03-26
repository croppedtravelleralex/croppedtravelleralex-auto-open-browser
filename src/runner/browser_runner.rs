use async_trait::async_trait;

use crate::{error::AppError, model::task::TaskRecord, state::AppState};

use super::{
    fake_runner::TaskRunner,
    lightpanda::{adapter::BrowserAdapter, models::BrowserRunResult},
};

pub struct BrowserRunner<A: BrowserAdapter> {
    adapter: A,
}

impl<A: BrowserAdapter> BrowserRunner<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    fn convert_result(result: BrowserRunResult) -> serde_json::Value {
        serde_json::json!({
            "result": result.result_json,
            "logs": result.logs,
            "artifacts": result.artifacts,
        })
    }
}

#[async_trait]
impl<A: BrowserAdapter> TaskRunner for BrowserRunner<A> {
    async fn run(&self, _state: &AppState, task: &TaskRecord) -> Result<serde_json::Value, AppError> {
        let result = self.adapter.run_task(task).await?;
        Ok(Self::convert_result(result))
    }
}
