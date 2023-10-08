use std::{env, net::SocketAddr, str::FromStr};

use migration::{Migrator, MigratorTrait};
use poem::{listener::TcpListener, web::Data, EndpointExt, Route, Server};
use poem_openapi::{payload::Json, Object, OpenApi, OpenApiService};
use sea_orm::{Database, DatabaseConnection};
use tracing::{event, span, Level};

struct Api;

#[derive(Object)]
struct Test;

#[OpenApi]
impl Api {
    /// List tasks
    #[oai(path = "/", method = "get")]
    async fn index(&self, conn: Data<&DatabaseConnection>) -> Json<Test> {
        Json(Test {})
    }
}

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

    let api_serive = OpenApiService::new(Api, "mini-do backend", "1.0").server(addr.to_string());
    let app = Route::new().nest("/", api_serive).data(conn);

    Server::new(TcpListener::bind(addr)).run(app).await.unwrap();

    drop(start_up_guard);

    Ok(())
}
