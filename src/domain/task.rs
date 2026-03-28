use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Succeeded => "succeeded",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
            TaskStatus::TimedOut => "timed_out",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Succeeded | TaskStatus::Failed | TaskStatus::Cancelled | TaskStatus::TimedOut
        )
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInput {
    pub url: Option<String>,
    pub script: Option<String>,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub kind: String,
    pub status: TaskStatus,
    pub priority: i32,
    pub input: TaskInput,
    pub created_at: String,
}
