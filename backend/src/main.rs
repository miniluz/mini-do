use std::env;

use sea_orm::Database;

use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dbg!(env::var("DATABASE_URL")?);
    let db = Database::connect(env::var("DATABASE_URL")?).await?;

    println!("All fine!");

    Ok(())
}
