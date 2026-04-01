use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        Self::from_json_str(&raw)
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

    pub fn ensure_default_state_file(path: impl AsRef<Path>, project: &str) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            match Self::load(path) {
                Ok(state) => return Ok(state),
                Err(_) => {
                    let raw = fs::read_to_string(path)
                        .with_context(|| format!("failed to read legacy workflow state from {}", path.display()))?;
                    let migrated = Self::from_json_str(&raw).unwrap_or_else(|_| Self::new(project));
                    migrated.save(path)?;
                    return Ok(migrated);
                }
            }
        }
        let state = Self::new(project);
        state.save(path)?;
        Ok(state)
    }

    fn from_json_str(raw: &str) -> Result<Self> {
        match serde_json::from_str::<Self>(raw) {
            Ok(state) => Ok(state),
            Err(_) => Self::from_legacy_value(serde_json::from_str(raw).context("failed to parse workflow json value")?),
        }
    }

    fn from_legacy_value(value: Value) -> Result<Self> {
        let stage = match value.get("nextRoundType").and_then(|v| v.as_str()).unwrap_or("plan") {
            "plan" => WorkflowStage::Plan,
            "build" => WorkflowStage::Execute,
            "verify" => WorkflowStage::Verify,
            "summarize" => WorkflowStage::DocSync,
            _ => WorkflowStage::Plan,
        };
        let next_suggestions = value
            .get("nextRecommendedActions")
            .and_then(|v| v.as_array())
            .map(|items| {
                items
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, item)| item.as_str().map(|title| WorkflowSuggestion {
                        title: title.to_string(),
                        priority: (idx + 1) as u8,
                        rationale: "从旧 RUN_STATE.json 迁移而来".to_string(),
                    }))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(Self {
            project: value.get("project").and_then(|v| v.as_str()).unwrap_or("AutoOpenBrowser").to_string(),
            loop_enabled: value.get("schedulerStatus").and_then(|v| v.as_str()) == Some("running"),
            loop_iteration: value.get("currentRound").and_then(|v| v.as_u64()).unwrap_or(0),
            stage,
            bug_cycle_interval: 3,
            completed_since_bug_cycle: 0,
            consecutive_failures: value.get("failureCount").and_then(|v| v.as_u64()).unwrap_or(0),
            current_focus: value.get("currentFocus").and_then(|v| v.as_str()).unwrap_or("迁移旧执行状态").to_string(),
            current_objective: value.get("currentObjective").and_then(|v| v.as_str()).unwrap_or("初始化新的工作流状态机").to_string(),
            last_result_summary: value.get("lastVerificationResult").and_then(|v| v.as_str()).unwrap_or("从旧 RUN_STATE.json 迁移").to_string(),
            next_action_hint: value.get("lastSchedulerDecision").and_then(|v| v.as_str()).unwrap_or("下一步进入计划阶段").to_string(),
            next_suggestions,
        })
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

    #[test]
    fn workflow_state_can_migrate_legacy_run_state() {
        let legacy = r#"{
          "project": "AutoOpenBrowser",
          "currentRound": 78,
          "roundType": "plan",
          "currentObjective": "进入 build 轮，新增具体 SQLite schema 设计文档。",
          "lastVerificationResult": "已完成新一轮 plan，锁定下一步为细化 SQLite schema 草案。",
          "failureCount": 2,
          "lastSchedulerDecision": "Executed plan, next=build",
          "nextRoundType": "build",
          "schedulerStatus": "running",
          "currentFocus": "周期执行协议已落地",
          "nextRecommendedActions": ["基于状态机进行 1 个 mini-cycle 试运行", "初始化 Cargo 工程"]
        }"#;
        let state = WorkflowExecutionState::from_json_str(legacy).expect("migrate legacy state");
        assert_eq!(state.loop_iteration, 78);
        assert_eq!(state.stage, WorkflowStage::Execute);
        assert_eq!(state.consecutive_failures, 2);
        assert_eq!(state.next_suggestions.len(), 2);
        assert!(state.loop_enabled);
    }

    #[test]
    fn ensure_default_state_file_creates_new_state() {
        let dir = tempdir().expect("tempdir");
        let path = dir.path().join("RUN_STATE.json");
        let state = WorkflowExecutionState::ensure_default_state_file(&path, "AutoOpenBrowser").expect("ensure state");
        assert_eq!(state.project, "AutoOpenBrowser");
        assert!(path.exists());
    }
}
