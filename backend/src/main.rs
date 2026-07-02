use anyhow::Result;
use axum::{Router, routing::get};
use std::env;

mod adapters;
mod client;
mod db;
mod product;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/api/hello", get(|| async { "Hello, World!" }))
        .route(
            "/api/refresh",
            get(|| async {
                match fetch_items().await {
                    Ok(_) => "Items fetched and saved successfully.",
                    Err(e) => {
                        eprintln!("Error fetching items: {:?}", e);
                        "Error fetching items."
                    }
                }
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn fetch_items() -> Result<()> {
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
