use async_trait::async_trait;

use crate::{error::AppError, model::task::TaskRecord};

use super::{adapter::BrowserAdapter, models::{BrowserRunResult, LightpandaCommand}};

#[derive(Debug, Clone)]
pub struct LightpandaCliAdapter {
    pub command: String,
}

impl Default for LightpandaCliAdapter {
    fn default() -> Self {
        Self {
            command: "lightpanda-browser".to_string(),
        }
    }
}

impl LightpandaCliAdapter {
    pub fn build_command_payload(&self, task: &TaskRecord) -> Result<LightpandaCommand, AppError> {
        Ok(LightpandaCommand {
            steps: task.input_json.clone(),
            timeout_seconds: task.timeout_seconds,
        })
    }
}

#[async_trait]
impl BrowserAdapter for LightpandaCliAdapter {
    async fn healthcheck(&self) -> Result<(), AppError> {
        Err(AppError::Runner(
            "lightpanda CLI adapter is not implemented yet".into(),
        ))
    }

    async fn run_task(&self, task: &TaskRecord) -> Result<BrowserRunResult, AppError> {
        let _payload = self.build_command_payload(task)?;
        Err(AppError::Runner(
            "lightpanda CLI adapter is not implemented yet".into(),
        ))
    }
}
