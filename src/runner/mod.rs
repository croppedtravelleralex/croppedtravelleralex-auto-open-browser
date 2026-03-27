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

pub async fn spawn_runner_loop(state: AppState, runner: Arc<dyn TaskRunner>) {
    tokio::spawn(async move {
        loop {
            let _ = engine::run_one_task_with_runner(&state, runner.as_ref()).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });
}
