use std::net::SocketAddr;

use anyhow::Result;
use axum::serve;
use tokio::net::TcpListener;

use AutoOpenBrowser::{
    api::routes::build_router,
    app::state::AppState,
    db::init::init_db,
    queue::memory::MemoryTaskQueue,
    runner::fake::spawn_fake_runner_loop,
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
    let state = AppState { db, queue, api_key };

    spawn_fake_runner_loop(state.clone()).await;

    let app = build_router(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("AutoOpenBrowser listening on http://{}", addr);
    println!("Database initialized at {}", database_url);
    serve(listener, app).await?;

    Ok(())
}
