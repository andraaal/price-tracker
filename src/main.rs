use anyhow::Result;
use std::env;

mod adapters;
mod client;
mod db;
mod product;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let db = db::DB::new(&database_url).await?;
    sqlx::migrate!().run(&db.pool).await?;

    let client = client::create_client()?;
    let items = adapters::spar::fetch_items(&client).await?;

    for item in items {
        println!("Saved product: {} from {}", item.name, item.brand);
        db.save_product(item).await?;
    }

    Ok(())
}
