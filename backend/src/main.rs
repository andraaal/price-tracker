use anyhow::Result;
use axum::{Router, extract::State, routing::get};
use std::env;

use crate::db::DB;

mod adapters;
mod client;
mod db;
mod product;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let db: DB = db::DB::new(&database_url).await?;
    sqlx::migrate!().run(&db.pool).await?;

    let app = Router::new()
        .route("/api/refresh", get(refresh))
        .with_state(db.clone())
        .route("/api/products", get(get_products))
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_products(State(db): State<DB>) -> axum::Json<Vec<product::Product>> {
    match db.get_all_products().await {
        Ok(products) => axum::Json(products),
        Err(e) => {
            eprintln!("Error fetching products: {:?}", e);
            axum::Json(Vec::<product::Product>::new())
        }
    }
}

async fn refresh(State(db): State<DB>) -> &'static str {
    match fetch_items(&db).await {
        Ok(_) => "Items fetched and saved successfully.",
        Err(e) => {
            eprintln!("Error fetching items: {:?}", e);
            "Error fetching items."
        }
    }
}

async fn fetch_items(db: &DB) -> Result<()> {
    let client = client::create_client()?;
    let items = adapters::interspar::fetch_items(&client).await?;

    for item in items {
        println!("Saved product: {} from {}", item.name, item.brand);
        db.save_product(item).await?;
    }

    Ok(())
}
