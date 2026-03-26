use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    config::AppConfig,
    scheduler::queue::QueuedTask,
    storage::sqlite::Database,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: Arc<Database>,
    pub task_tx: mpsc::Sender<QueuedTask>,
}
