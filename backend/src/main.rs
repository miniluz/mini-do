use std::env;

use sea_orm::Database;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::connect(format!(
        "postgres://{}:{}@db/{}",
        env::var("POSTGRES_USER").expect("POSTGRES_USER expected in env"),
        env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD expected in env"),
        env::var("POSTGRES_DB").expect("POSTGRES_DB expected in env"),
    ))
    .await?;

    println!("All fine!");

    Ok(())
}
