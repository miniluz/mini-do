use std::{env, net::SocketAddr, str::FromStr};

mod entities;

use migration::{Migrator, MigratorTrait};
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sea_orm::Database;
use tracing::{event, span, Level};

mod task;

use task::TaskApi;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in env.");

    let start_up_span = span!(Level::INFO, "start-up");
    let start_up_guard = start_up_span.enter();

    event!(Level::INFO, "Connecting to database...");
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database.");
    event!(Level::INFO, "Connected to database successfully");

    event!(Level::INFO, "Trying to migrate...");
    Migrator::up(&conn, None).await.unwrap();
    event!(Level::INFO, "Migrated");

    event!(Level::INFO, "Starting server...");
    let host = env::var("HOST").expect("HOST is not set in env.");
    let port = env::var("PORT").expect("PORT is not set in env.");
    let addr = SocketAddr::from_str(&format!("{host}:{port}")).unwrap();

    let task_service =
        OpenApiService::new(TaskApi, "Task endpoint", "1.0").server(addr.to_string());
    let app = Route::new().nest("/backend", task_service).data(conn);

    drop(start_up_guard);

    Server::new(TcpListener::bind(addr)).run(app).await.unwrap();

    Ok(())
}
