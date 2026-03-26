use std::{fs, path::Path, sync::Mutex};

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde_json::Value;

use crate::{
    error::AppError,
    model::task::{TaskLogRecord, TaskRecord, TaskStatus},
};

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, AppError> {
        if let Some(parent) = Path::new(db_path).parent() {
            fs::create_dir_all(parent).map_err(|e| AppError::Database(e.to_string()))?;
        }

        let conn = Connection::open(db_path).map_err(|e| AppError::Database(e.to_string()))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn init(&self) -> Result<(), AppError> {
        let sql = include_str!("../../migrations/001_init.sql");
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute_batch(sql)
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn insert_task(&self, task: &TaskRecord) -> Result<(), AppError> {
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO tasks (id, name, status, created_at, started_at, finished_at, timeout_seconds, input_json, result_json, error_text, artifact_dir)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                task.id,
                task.name,
                task.status.as_str(),
                task.created_at.to_rfc3339(),
                task.started_at.map(|v| v.to_rfc3339()),
                task.finished_at.map(|v| v.to_rfc3339()),
                task.timeout_seconds,
                task.input_json.to_string(),
                task.result_json.as_ref().map(|v| v.to_string()),
                task.error_text,
                task.artifact_dir,
            ],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn get_task(&self, task_id: &str) -> Result<TaskRecord, AppError> {
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        let mut stmt = conn
            .prepare("SELECT id, name, status, created_at, started_at, finished_at, timeout_seconds, input_json, result_json, error_text, artifact_dir FROM tasks WHERE id = ?1")
            .map_err(|e| AppError::Database(e.to_string()))?;

        let row_data = stmt
            .query_row([task_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                ))
            })
            .map_err(|_| AppError::NotFound)?;

        let task = TaskRecord {
            id: row_data.0,
            name: row_data.1,
            status: parse_status(&row_data.2),
            created_at: parse_dt(&row_data.3)?,
            started_at: parse_optional_dt(row_data.4)?,
            finished_at: parse_optional_dt(row_data.5)?,
            timeout_seconds: row_data.6,
            input_json: serde_json::from_str(&row_data.7).unwrap_or(Value::Null),
            result_json: row_data.8.and_then(|v| serde_json::from_str(&v).ok()),
            error_text: row_data.9,
            artifact_dir: row_data.10,
        };

        Ok(task)
    }

    pub fn list_tasks(&self) -> Result<Vec<TaskRecord>, AppError> {
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        let mut stmt = conn
            .prepare("SELECT id, name, status, created_at, started_at, finished_at, timeout_seconds, input_json, result_json, error_text, artifact_dir FROM tasks ORDER BY created_at DESC")
            .map_err(|e| AppError::Database(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                ))
            })
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut tasks = Vec::new();
        for row in rows {
            let row = row.map_err(|e| AppError::Database(e.to_string()))?;
            tasks.push(TaskRecord {
                id: row.0,
                name: row.1,
                status: parse_status(&row.2),
                created_at: parse_dt(&row.3)?,
                started_at: parse_optional_dt(row.4)?,
                finished_at: parse_optional_dt(row.5)?,
                timeout_seconds: row.6,
                input_json: serde_json::from_str(&row.7).unwrap_or(Value::Null),
                result_json: row.8.and_then(|v| serde_json::from_str(&v).ok()),
                error_text: row.9,
                artifact_dir: row.10,
            });
        }

        Ok(tasks)
    }

    pub fn mark_running(&self, task_id: &str) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute(
            "UPDATE tasks SET status = 'running', started_at = ?2 WHERE id = ?1",
            params![task_id, now],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn mark_success(&self, task_id: &str, result: &Value) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute(
            "UPDATE tasks SET status = 'success', finished_at = ?2, result_json = ?3 WHERE id = ?1",
            params![task_id, now, result.to_string()],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn mark_failed(&self, task_id: &str, error: &str) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute(
            "UPDATE tasks SET status = 'failed', finished_at = ?2, error_text = ?3 WHERE id = ?1",
            params![task_id, now, error],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn append_log(&self, task_id: &str, level: &str, message: &str) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        conn.execute(
            "INSERT INTO task_logs (task_id, ts, level, message) VALUES (?1, ?2, ?3, ?4)",
            params![task_id, now, level, message],
        )
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub fn get_logs(&self, task_id: &str) -> Result<Vec<TaskLogRecord>, AppError> {
        let conn = self.conn.lock().map_err(|_| AppError::Database("db mutex poisoned".into()))?;
        let mut stmt = conn
            .prepare("SELECT id, task_id, ts, level, message FROM task_logs WHERE task_id = ?1 ORDER BY id ASC")
            .map_err(|e| AppError::Database(e.to_string()))?;

        let rows = stmt
            .query_map([task_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut logs = Vec::new();
        for row in rows {
            let row = row.map_err(|e| AppError::Database(e.to_string()))?;
            logs.push(TaskLogRecord {
                id: row.0,
                task_id: row.1,
                ts: parse_dt(&row.2)?,
                level: row.3,
                message: row.4,
            });
        }

        Ok(logs)
    }
}

fn parse_status(s: &str) -> TaskStatus {
    match s {
        "queued" => TaskStatus::Queued,
        "running" => TaskStatus::Running,
        "success" => TaskStatus::Success,
        "failed" => TaskStatus::Failed,
        "cancelled" => TaskStatus::Cancelled,
        "timeout" => TaskStatus::Timeout,
        _ => TaskStatus::Failed,
    }
}

fn parse_dt(s: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(s)
        .map(|v| v.with_timezone(&Utc))
        .map_err(|e| AppError::Database(e.to_string()))
}

fn parse_optional_dt(s: Option<String>) -> Result<Option<DateTime<Utc>>, AppError> {
    match s {
        Some(v) => Ok(Some(parse_dt(&v)?)),
        None => Ok(None),
    }
}
