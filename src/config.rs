#[derive(Debug, Clone, Copy)]
pub enum RunnerModeConfig {
    Fake,
    Browser,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub listen_addr: String,
    pub data_dir: String,
    pub db_path: String,
    pub artifacts_dir: String,
    pub max_concurrent_tasks: usize,
    pub default_timeout_seconds: i64,
    pub runner_mode: RunnerModeConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = "data".to_string();
        let artifacts_dir = format!("{data_dir}/artifacts");
        let db_path = format!("{data_dir}/app.db");

        Self {
            listen_addr: "127.0.0.1:8080".to_string(),
            data_dir,
            db_path,
            artifacts_dir,
            max_concurrent_tasks: 4,
            default_timeout_seconds: 60,
            runner_mode: RunnerModeConfig::Fake,
        }
    }
}
