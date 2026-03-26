use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserRunResult {
    pub result_json: Value,
    pub logs: Vec<String>,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightpandaCommand {
    pub steps: Value,
    pub timeout_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightpandaRawOutput {
    pub ok: bool,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub result: Option<Value>,
}
