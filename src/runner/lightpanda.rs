use async_trait::async_trait;
use serde_json::{json, Value};
use std::{collections::HashMap, process::Stdio, sync::{Arc, Mutex}};
use tokio::{process::{Child, Command}, time::{timeout, Duration}};

use crate::runner::{
    RunnerCancelResult, RunnerCapabilities, RunnerExecutionResult, RunnerOutcomeStatus,
    RunnerTask, TaskRunner,
};

#[derive(Clone, Default)]
pub struct LightpandaRunner {
    running_tasks: Arc<Mutex<HashMap<String, u32>>>,
}

fn result_payload(
    ok: bool,
    status: &str,
    error_kind: Option<&str>,
    url: Option<&str>,
    timeout_seconds: Option<u64>,
    bin: Option<&str>,
    exit_code: Option<i32>,
    stdout_preview: Option<String>,
    stderr_preview: Option<String>,
    message: &str,
) -> Value {
    json!({
        "runner": "lightpanda",
        "action": "open_page",
        "ok": ok,
        "status": status,
        "error_kind": error_kind,
        "url": url,
        "timeout_seconds": timeout_seconds,
        "bin": bin,
        "exit_code": exit_code,
        "stdout_preview": stdout_preview,
        "stderr_preview": stderr_preview,
        "message": message,
    })
}

fn invalid_input(message: &str, url: Option<&str>) -> RunnerExecutionResult {
    RunnerExecutionResult {
        status: RunnerOutcomeStatus::Failed,
        result_json: Some(result_payload(
            false,
            "failed",
            Some("invalid_input"),
            url,
            None,
            None,
            None,
            None,
            None,
            message,
        )),
        error_message: Some(message.to_string()),
    }
}

fn extract_url(payload: &Value) -> Option<String> {
    payload
        .get("url")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn looks_like_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

fn lightpanda_bin() -> String {
    std::env::var("LIGHTPANDA_BIN")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "lightpanda".to_string())
}

fn truncate_output(s: &str, max_chars: usize) -> String {
    let trimmed = s.trim();
    let mut out: String = trimmed.chars().take(max_chars).collect();
    if trimmed.chars().count() > max_chars {
        out.push_str("...[truncated]");
    }
    out
}

async fn terminate_pid(pid: u32) -> Result<(), String> {
    let status = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .await
        .map_err(|err| format!("failed to spawn kill command: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("kill -TERM exited with status {:?}", status.code()))
    }
}

impl LightpandaRunner {
    fn register_child(&self, task_id: &str, child: &Child) {
        if let Some(pid) = child.id() {
            let mut guard = self.running_tasks.lock().expect("lightpanda running_tasks poisoned");
            guard.insert(task_id.to_string(), pid);
        }
    }

    fn unregister_child(&self, task_id: &str) {
        let mut guard = self.running_tasks.lock().expect("lightpanda running_tasks poisoned");
        guard.remove(task_id);
    }
}

#[async_trait]
impl TaskRunner for LightpandaRunner {
    fn name(&self) -> &'static str {
        "lightpanda"
    }

    fn capabilities(&self) -> RunnerCapabilities {
        RunnerCapabilities {
            supports_timeout: true,
            supports_cancel_running: true,
            supports_artifacts: false,
        }
    }

    async fn cancel_running(&self, task_id: &str) -> RunnerCancelResult {
        let pid = {
            let guard = self.running_tasks.lock().expect("lightpanda running_tasks poisoned");
            guard.get(task_id).copied()
        };

        match pid {
            Some(pid) => match terminate_pid(pid).await {
                Ok(()) => {
                    self.unregister_child(task_id);
                    RunnerCancelResult {
                        accepted: true,
                        message: format!(
                            "lightpanda runner sent SIGTERM to running process for task_id={task_id}, pid={pid}"
                        ),
                    }
                }
                Err(err) => RunnerCancelResult {
                    accepted: false,
                    message: format!(
                        "lightpanda runner failed to terminate process for task_id={task_id}, pid={pid}: {err}"
                    ),
                },
            },
            None => RunnerCancelResult {
                accepted: false,
                message: format!(
                    "lightpanda runner has no registered running process for task_id={task_id}"
                ),
            },
        }
    }

    async fn execute(&self, task: RunnerTask) -> RunnerExecutionResult {
        let url = match extract_url(&task.payload) {
            Some(url) => url,
            None => {
                return invalid_input(
                    "lightpanda runner requires a non-empty url in task payload",
                    None,
                )
            }
        };

        if !looks_like_url(&url) {
            return invalid_input(
                "lightpanda runner currently only accepts http:// or https:// urls",
                Some(&url),
            );
        }

        let timeout_seconds = task.timeout_seconds.unwrap_or(10).clamp(1, 120) as u64;
        let bin = lightpanda_bin();

        let mut cmd = Command::new(&bin);
        cmd.arg("fetch")
            .arg("--log-format")
            .arg("pretty")
            .arg("--log-level")
            .arg("info")
            .arg(&url)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = match cmd.spawn() {
            Ok(child) => child,
            Err(err) => {
                let message = format!("failed to spawn lightpanda binary: {err}");
                return RunnerExecutionResult {
                    status: RunnerOutcomeStatus::Failed,
                    result_json: Some(result_payload(
                        false,
                        "failed",
                        Some("spawn_failed"),
                        Some(&url),
                        Some(timeout_seconds),
                        Some(&bin),
                        None,
                        None,
                        None,
                        &message,
                    )),
                    error_message: Some(message),
                };
            }
        };

        self.register_child(&task.task_id, &child);

        let output = match timeout(Duration::from_secs(timeout_seconds), child.wait_with_output()).await {
            Ok(result) => match result {
                Ok(output) => output,
                Err(err) => {
                    self.unregister_child(&task.task_id);
                    let message = format!("lightpanda process wait failed: {err}");
                    return RunnerExecutionResult {
                        status: RunnerOutcomeStatus::Failed,
                        result_json: Some(result_payload(
                            false,
                            "failed",
                            Some("process_wait_failed"),
                            Some(&url),
                            Some(timeout_seconds),
                            Some(&bin),
                            None,
                            None,
                            None,
                            &message,
                        )),
                        error_message: Some(message),
                    };
                }
            },
            Err(_) => {
                self.unregister_child(&task.task_id);
                let message = "lightpanda fetch timed out";
                return RunnerExecutionResult {
                    status: RunnerOutcomeStatus::TimedOut,
                    result_json: Some(result_payload(
                        false,
                        "timeout",
                        Some("timeout"),
                        Some(&url),
                        Some(timeout_seconds),
                        Some(&bin),
                        None,
                        None,
                        None,
                        message,
                    )),
                    error_message: Some(message.to_string()),
                };
            }
        };

        self.unregister_child(&task.task_id);

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout_preview = truncate_output(&stdout, 4000);
        let stderr_preview = truncate_output(&stderr, 2000);
        let exit_code = output.status.code();

        if output.status.success() {
            let message = "lightpanda fetch completed successfully";
            RunnerExecutionResult {
                status: RunnerOutcomeStatus::Succeeded,
                result_json: Some(result_payload(
                    true,
                    "succeeded",
                    None,
                    Some(&url),
                    Some(timeout_seconds),
                    Some(&bin),
                    exit_code,
                    Some(stdout_preview),
                    if stderr_preview.is_empty() { None } else { Some(stderr_preview) },
                    message,
                )),
                error_message: None,
            }
        } else {
            let message = "lightpanda fetch exited with non-zero status";
            RunnerExecutionResult {
                status: RunnerOutcomeStatus::Failed,
                result_json: Some(result_payload(
                    false,
                    "failed",
                    Some("non_zero_exit"),
                    Some(&url),
                    Some(timeout_seconds),
                    Some(&bin),
                    exit_code,
                    if stdout_preview.is_empty() { None } else { Some(stdout_preview) },
                    if stderr_preview.is_empty() { None } else { Some(stderr_preview) },
                    message,
                )),
                error_message: Some(message.to_string()),
            }
        }
    }
}
