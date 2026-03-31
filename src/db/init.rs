use std::path::Path;

use anyhow::Result;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite};
use tokio::fs;

use super::schema::ALL_SCHEMA_SQL;

pub type DbPool = Pool<Sqlite>;

async fn ensure_column_exists(pool: &DbPool, table: &str, column: &str, alter_sql: &str) -> Result<()> {
    let pragma = format!("PRAGMA table_info({})", table);
    let rows = sqlx::query_as::<_, (i64, String, String, i64, Option<String>, i64)>(&pragma)
        .fetch_all(pool)
        .await?;
    let exists = rows.into_iter().any(|(_, name, _, _, _, _)| name == column);
    if !exists {
        sqlx::query(alter_sql).execute(pool).await?;
    }
    Ok(())
}

async fn ensure_sqlite_parent_dir(database_url: &str) -> Result<()> {
    if let Some(path_str) = database_url.strip_prefix("sqlite://") {
        let path = Path::new(path_str);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).await?;
            }
        }
    }
    Ok(())
}

pub async fn init_db(database_url: &str) -> Result<DbPool> {
    ensure_sqlite_parent_dir(database_url).await?;

    let options = database_url
        .parse::<SqliteConnectOptions>()?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    for stmt in ALL_SCHEMA_SQL {
        sqlx::query(stmt).execute(&pool).await?;
    }

    ensure_column_exists(&pool, "tasks", "runner_id", "ALTER TABLE tasks ADD COLUMN runner_id TEXT").await?;
    ensure_column_exists(&pool, "tasks", "heartbeat_at", "ALTER TABLE tasks ADD COLUMN heartbeat_at TEXT").await?;
    ensure_column_exists(&pool, "tasks", "fingerprint_profile_id", "ALTER TABLE tasks ADD COLUMN fingerprint_profile_id TEXT").await?;
    ensure_column_exists(&pool, "tasks", "fingerprint_profile_version", "ALTER TABLE tasks ADD COLUMN fingerprint_profile_version INTEGER").await?;
    ensure_column_exists(&pool, "proxies", "last_probe_latency_ms", "ALTER TABLE proxies ADD COLUMN last_probe_latency_ms INTEGER").await?;
    ensure_column_exists(&pool, "proxies", "last_probe_error", "ALTER TABLE proxies ADD COLUMN last_probe_error TEXT").await?;
    ensure_column_exists(&pool, "proxies", "last_probe_error_category", "ALTER TABLE proxies ADD COLUMN last_probe_error_category TEXT").await?;
    ensure_column_exists(&pool, "proxies", "last_verify_confidence", "ALTER TABLE proxies ADD COLUMN last_verify_confidence REAL").await?;
    ensure_column_exists(&pool, "proxies", "last_verify_score_delta", "ALTER TABLE proxies ADD COLUMN last_verify_score_delta INTEGER").await?;
    ensure_column_exists(&pool, "proxies", "last_verify_source", "ALTER TABLE proxies ADD COLUMN last_verify_source TEXT").await?;

    Ok(pool)
}
