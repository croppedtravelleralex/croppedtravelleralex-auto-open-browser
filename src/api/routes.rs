use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;

use crate::{api::handlers::{create_task, get_task, get_task_logs, health, list_tasks}, state::AppState};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", post(create_task).get(list_tasks))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id/logs", get(get_task_logs))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
