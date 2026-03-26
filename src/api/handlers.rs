use std::sync::Arc;

use axum::{extract::{Path, State}, Json};
use chrono::Utc;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    error::AppError,
    model::task::{CreateTaskRequest, CreateTaskResponse, TaskRecord, TaskStatus},
    scheduler::queue::QueuedTask,
    state::AppState,
};

pub async fn health() -> Json<Value> {
    Json(json!({ "ok": true }))
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>, AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::InvalidRequest("name cannot be empty".into()));
    }

    if payload.steps.is_empty() {
        return Err(AppError::InvalidRequest("steps cannot be empty".into()));
    }

    let task_id = Uuid::new_v4().to_string();
    let task = TaskRecord {
        id: task_id.clone(),
        name: payload.name,
        status: TaskStatus::Queued,
        created_at: Utc::now(),
        started_at: None,
        finished_at: None,
        timeout_seconds: payload
            .timeout_seconds
            .unwrap_or(state.config.default_timeout_seconds),
        input_json: json!({
            "steps": payload.steps,
        }),
        result_json: None,
        error_text: None,
        artifact_dir: None,
    };

    state.db.insert_task(&task)?;
    state.db.append_log(&task.id, "info", "task created")?;
    state
        .task_tx
        .send(QueuedTask {
            task_id: task.id.clone(),
        })
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(CreateTaskResponse {
        task_id,
        status: TaskStatus::Queued,
    }))
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskRecord>, AppError> {
    let task = state.db.get_task(&task_id)?;
    Ok(Json(task))
}

pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TaskRecord>>, AppError> {
    let tasks = state.db.list_tasks()?;
    Ok(Json(tasks))
}

pub async fn get_task_logs(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let logs = state.db.get_logs(&task_id)?;
    Ok(Json(json!(logs)))
}
