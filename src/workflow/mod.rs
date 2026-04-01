use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const DEFAULT_WORKFLOW_STATE_PATH: &str = "RUN_STATE.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStage {
    Plan,
    Execute,
    Verify,
    BugScan,
    BugFix,
    DocSync,
    CommitPush,
    Cooldown,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowSuggestion {
    pub title: String,
    pub priority: u8,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowExecutionState {
    pub project: String,
    pub loop_enabled: bool,
    pub loop_iteration: u64,
    pub stage: WorkflowStage,
    pub bug_cycle_interval: u64,
    pub completed_since_bug_cycle: u64,
    pub consecutive_failures: u64,
    pub current_focus: String,
    pub current_objective: String,
    pub last_result_summary: String,
    pub next_action_hint: String,
    pub next_suggestions: Vec<WorkflowSuggestion>,
}

impl WorkflowExecutionState {
    pub fn new(project: impl Into<String>) -> Self {
        Self {
            project: project.into(),
            loop_enabled: false,
            loop_iteration: 0,
            stage: WorkflowStage::Plan,
            bug_cycle_interval: 3,
            completed_since_bug_cycle: 0,
            consecutive_failures: 0,
            current_focus: "建立自动执行工作流状态机骨架".to_string(),
            current_objective: "初始化 workflow state 文件与阶段枚举".to_string(),
            last_result_summary: "尚未开始自动循环".to_string(),
            next_action_hint: "先进入 plan 阶段，读取目标文档并生成建议".to_string(),
            next_suggestions: Vec::new(),
        }
    }

    pub fn should_enter_bug_cycle(&self) -> bool {
        self.completed_since_bug_cycle >= self.bug_cycle_interval || self.consecutive_failures > 0
    }

    pub fn advance_after_success(&mut self) {
        self.loop_iteration += 1;
        self.consecutive_failures = 0;
        self.completed_since_bug_cycle += 1;
        self.stage = if self.should_enter_bug_cycle() {
            WorkflowStage::BugScan
        } else {
            match self.stage {
                WorkflowStage::Plan => WorkflowStage::Execute,
                WorkflowStage::Execute => WorkflowStage::Verify,
                WorkflowStage::Verify => WorkflowStage::DocSync,
                WorkflowStage::BugScan => WorkflowStage::BugFix,
                WorkflowStage::BugFix => {
                    self.completed_since_bug_cycle = 0;
                    WorkflowStage::CommitPush
                }
                WorkflowStage::DocSync => WorkflowStage::Cooldown,
                WorkflowStage::CommitPush => WorkflowStage::Cooldown,
                WorkflowStage::Cooldown => WorkflowStage::Plan,
                WorkflowStage::Blocked => WorkflowStage::Plan,
            }
        };
    }

    pub fn mark_failure(&mut self, summary: impl Into<String>) {
        self.consecutive_failures += 1;
        self.last_result_summary = summary.into();
        self.stage = WorkflowStage::BugScan;
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read workflow state from {}", path.display()))?;
        serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse workflow state from {}", path.display()))
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create workflow state dir {}", parent.display()))?;
        let text = serde_json::to_string_pretty(self).context("failed to serialize workflow state")?;
        fs::write(path, text)
            .with_context(|| format!("failed to write workflow state to {}", path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn workflow_state_roundtrip_works() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("RUN_STATE.json");
        let mut state = WorkflowExecutionState::new("AutoOpenBrowser");
        state.loop_enabled = true;
        state.next_suggestions = vec![WorkflowSuggestion {
            title: "实现工作流状态机骨架".to_string(),
            priority: 1,
            rationale: "这是自动循环执行器的起点".to_string(),
        }];
        state.save(&path).expect("save state");
        let loaded = WorkflowExecutionState::load(&path).expect("load state");
        assert_eq!(loaded, state);
    }

    #[test]
    fn workflow_state_enters_bug_cycle_after_threshold() {
        let mut state = WorkflowExecutionState::new("AutoOpenBrowser");
        state.completed_since_bug_cycle = 3;
        assert!(state.should_enter_bug_cycle());
        state.advance_after_success();
        assert_eq!(state.stage, WorkflowStage::BugScan);
    }

    #[test]
    fn workflow_state_failure_redirects_to_bug_scan() {
        let mut state = WorkflowExecutionState::new("AutoOpenBrowser");
        state.stage = WorkflowStage::Execute;
        state.mark_failure("integration test failed");
        assert_eq!(state.stage, WorkflowStage::BugScan);
        assert_eq!(state.consecutive_failures, 1);
        assert_eq!(state.last_result_summary, "integration test failed");
    }
}
