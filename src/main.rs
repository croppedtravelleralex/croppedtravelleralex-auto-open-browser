mod api;
mod config;
mod error;
mod model;
mod runner;
mod scheduler;
mod state;
mod storage;

use std::{fs, sync::Arc};

use config::AppConfig;
use scheduler::dispatcher::start_dispatcher;
use state::AppState;
use storage::sqlite::Database;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();

    let config = AppConfig::default();
    ensure_dirs(&config);

    let db = Arc::new(Database::new(&config.db_path).expect("failed to create database"));
    db.init().expect("failed to initialize database");

    let (task_tx, task_rx) = mpsc::channel(1024);

    let state = Arc::new(AppState {
        config: config.clone(),
        db,
        task_tx,
    });

    tokio::spawn(start_dispatcher(state.clone(), task_rx));

    let app = api::routes::routes(state);
    let listener = tokio::net::TcpListener::bind(&config.listen_addr)
        .await
        .expect("failed to bind listener");

    info!(addr = %config.listen_addr, "lightpanda automation service listening");
    axum::serve(listener, app)
        .await
        .expect("server error");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lightpanda_automation=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn ensure_dirs(config: &AppConfig) {
    fs::create_dir_all(&config.data_dir).expect("failed to create data dir");
    fs::create_dir_all(&config.artifacts_dir).expect("failed to create artifacts dir");
}
