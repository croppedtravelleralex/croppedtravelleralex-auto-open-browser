use std::sync::Arc;

use tokio::sync::{mpsc, Semaphore};
use tracing::error;

use crate::{
    config::RunnerModeConfig,
    runner::{
        browser_runner::BrowserRunner,
        fake_runner::{FakeRunner, TaskRunner},
        lightpanda::cli::LightpandaCliAdapter,
    },
    scheduler::queue::QueuedTask,
    state::AppState,
};

pub async fn start_dispatcher(state: Arc<AppState>, mut rx: mpsc::Receiver<QueuedTask>) {
    let semaphore = Arc::new(Semaphore::new(state.config.max_concurrent_tasks));
    let runner: Arc<dyn TaskRunner> = build_runner(state.config.runner_mode);

    while let Some(job) = rx.recv().await {
        let permit = match semaphore.clone().acquire_owned().await {
            Ok(v) => v,
            Err(err) => {
                error!(?err, "failed to acquire semaphore");
                continue;
            }
        };

        let state = state.clone();
        let runner = runner.clone();

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(err) = process_one_task(state, runner, job).await {
                error!(error = %err, "task processing failed");
            }
        });
    }
}

fn build_runner(mode: RunnerModeConfig) -> Arc<dyn TaskRunner> {
    match mode {
        RunnerModeConfig::Fake => Arc::new(FakeRunner::new()),
        RunnerModeConfig::Browser => Arc::new(BrowserRunner::new(LightpandaCliAdapter::default())),
    }
}

async fn process_one_task(
    state: Arc<AppState>,
    runner: Arc<dyn TaskRunner>,
    job: QueuedTask,
) -> Result<(), crate::error::AppError> {
    state.db.mark_running(&job.task_id)?;
    state.db.append_log(&job.task_id, "info", "dispatcher: task marked running")?;

    let task = state.db.get_task(&job.task_id)?;
    match runner.run(&state, &task).await {
        Ok(result) => {
            state.db.mark_success(&job.task_id, &result)?;
        }
        Err(err) => {
            state.db.mark_failed(&job.task_id, &err.to_string())?;
        }
    }

    Ok(())
}
