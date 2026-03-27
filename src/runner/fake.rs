use async_trait::async_trait;
use serde_json::json;
use tokio::time::{sleep, Duration};

use crate::runner::{RunnerExecutionResult, RunnerOutcomeStatus, RunnerTask, TaskRunner};

pub struct FakeRunner;

#[async_trait]
impl TaskRunner for FakeRunner {
    fn name(&self) -> &'static str {
        "fake"
    }

    async fn execute(&self, task: RunnerTask) -> RunnerExecutionResult {
        let _ = task.timeout_seconds;
        sleep(Duration::from_millis(300)).await;

        match task.kind.as_str() {
            "fail" => RunnerExecutionResult {
                status: RunnerOutcomeStatus::Failed,
                result_json: None,
                error_message: Some("simulated failure by fake runner".to_string()),
            },
            "timeout" => RunnerExecutionResult {
                status: RunnerOutcomeStatus::TimedOut,
                result_json: None,
                error_message: Some("simulated timeout by fake runner".to_string()),
            },
            _ => RunnerExecutionResult {
                status: RunnerOutcomeStatus::Succeeded,
                result_json: Some(json!({
                    "runner": self.name(),
                    "message": "task completed by fake runner",
                    "task_id": task.task_id,
                    "attempt": task.attempt,
                    "payload": task.payload,
                })),
                error_message: None,
            },
        }
    }
}
