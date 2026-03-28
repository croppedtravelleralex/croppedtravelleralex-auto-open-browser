use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::serve;
use tokio::net::TcpListener;

use AutoOpenBrowser::{
    api::routes::build_router,
    app::state::AppState,
    db::init::init_db,
    queue::memory::MemoryTaskQueue,
    runner::{
        fake::FakeRunner, lightpanda::LightpandaRunner, runner_concurrency_from_env,
        runner_reclaim_seconds_from_env, spawn_runner_workers, RunnerKind, TaskRunner,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = "sqlite://data/auto_open_browser.db";
    let db = init_db(database_url).await?;
    let queue = MemoryTaskQueue::new();
    let api_key = std::env::var("AUTO_OPEN_BROWSER_API_KEY")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let runner: Arc<dyn TaskRunner> = match RunnerKind::from_env() {
        RunnerKind::Fake => Arc::new(FakeRunner),
        RunnerKind::Lightpanda => Arc::new(LightpandaRunner::default()),
    };

    let worker_count = runner_concurrency_from_env();
    let state = AppState {
        db,
        queue,
        api_key,
        runner: runner.clone(),
        worker_count,
    };

    spawn_runner_workers(state.clone(), runner, worker_count).await;

    let app = build_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("AutoOpenBrowser listening on http://{}", addr);
    println!("Database initialized at {}", database_url);
    println!("Runner kind: {:?}", RunnerKind::from_env());
    println!("Runner concurrency: {}", worker_count);
    println!("Runner reclaim after: {:?}", runner_reclaim_seconds_from_env());
    serve(listener, app).await?;

    Ok(())
}
