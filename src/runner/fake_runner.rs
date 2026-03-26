use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;
use tokio::time::sleep;

use crate::{error::AppError, model::task::TaskRecord, state::AppState};

#[async_trait]
pub trait TaskRunner: Send + Sync {
    async fn run(&self, state: &AppState, task: &TaskRecord) -> Result<serde_json::Value, AppError>;
}

#[derive(Default)]
pub struct FakeRunner;

impl FakeRunner {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl TaskRunner for FakeRunner {
    async fn run(&self, state: &AppState, task: &TaskRecord) -> Result<serde_json::Value, AppError> {
        state.db.append_log(&task.id, "info", "fake runner: starting task")?;
        sleep(Duration::from_millis(300)).await;
        state.db.append_log(&task.id, "info", "fake runner: simulating step execution")?;
        sleep(Duration::from_millis(500)).await;
        state.db.append_log(&task.id, "info", "fake runner: completed")?;

        Ok(json!({
            "message": "fake runner executed",
            "task_name": task.name,
            "steps_count": task.input_json.get("steps").and_then(|v| v.as_array()).map(|v| v.len()).unwrap_or(0)
        }))
    }
}
