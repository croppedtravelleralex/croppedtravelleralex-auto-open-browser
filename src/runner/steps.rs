use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Step {
    Goto { url: String },
    Wait { ms: u64 },
    Screenshot { name: String },
    Click { selector: String },
    Type { selector: String, text: String },
    ExtractText { selector: String, save_as: String },
    Evaluate { script: String, save_as: Option<String> },
}
