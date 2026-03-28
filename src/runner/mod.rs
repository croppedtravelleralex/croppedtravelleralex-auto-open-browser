pub mod engine;
pub mod fake;
pub mod lightpanda;
pub mod types;

use std::sync::Arc;

use async_trait::async_trait;

use crate::app::state::AppState;
pub use types::{
    RunnerCancelResult, RunnerCapabilities, RunnerExecutionResult, RunnerOutcomeStatus,
    RunnerTask,
};

#[derive(Debug, Clone, Copy)]
pub enum RunnerKind {
    Fake,
    Lightpanda,
}

impl RunnerKind {
    pub fn from_env() -> Self {
        match std::env::var("AUTO_OPEN_BROWSER_RUNNER")
            .ok()
            .unwrap_or_else(|| "fake".to_string())
            .to_ascii_lowercase()
            .as_str()
        {
            "lightpanda" => RunnerKind::Lightpanda,
            _ => RunnerKind::Fake,
        }
    }
}

pub fn runner_concurrency_from_env() -> usize {
    std::env::var("AUTO_OPEN_BROWSER_RUNNER_CONCURRENCY")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(1)
}

#[async_trait]
pub trait TaskRunner: Send + Sync {
    fn name(&self) -> &'static str;

    fn capabilities(&self) -> RunnerCapabilities {
        RunnerCapabilities {
            supports_timeout: true,
            supports_cancel_running: false,
            supports_artifacts: false,
        }
    }

    async fn execute(&self, task: RunnerTask) -> RunnerExecutionResult;

    async fn cancel_running(&self, task_id: &str) -> RunnerCancelResult {
        let _ = task_id;
        RunnerCancelResult {
            accepted: false,
            message: format!("runner {} does not support running cancel", self.name()),
        }
    }
}

pub async fn spawn_runner_workers(state: AppState, runner: Arc<dyn TaskRunner>, worker_count: usize) {
    let worker_count = worker_count.max(1);

    for worker_id in 0..worker_count {
        let state = state.clone();
        let runner = runner.clone();
        tokio::spawn(async move {
            loop {
                let _ = engine::run_one_task_with_runner(&state, runner.as_ref()).await;
                let _ = worker_id;
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });
    }
}
