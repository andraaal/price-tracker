use anyhow::Result;
use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use std::env;

use crate::{
    db::DB,
    filter::{Filter, Sort},
    product::{tag::Tag, vendor::Vendor},
};

mod adapters;
mod client;
mod db;
mod filter;
mod product;

#[derive(serde::Deserialize, Clone, Debug)]
struct FilterParams {
    page: Option<u32>,
    page_length: Option<u32>,

    search_string: Option<String>,
    vendor: Option<Vendor>,
    tag: Option<Tag>,
    max_price: Option<i16>,
    sort: Option<Sort>,
}

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

async fn get_products(
    State(db): State<DB>,
    Query(query): Query<FilterParams>,
) -> axum::Json<Vec<product::Product>> {
    let page = query.page.unwrap_or(1) - 1;
    let len = query.page_length.unwrap_or(20);
    let filter = Filter {
        search_string: query.search_string,
        vendor: query.vendor,
        tag: query.tag,
        max_price: query.max_price,
        sort: query.sort,
    };
    println!("Fetching products with filter: {:?}", filter);
    match db
        .get_products(page as i32 * len as i32, len as i32, filter)
        .await
    {
        Ok(products) => axum::Json(products),
        Err(e) => {
            eprintln!("Error fetching products: {:?}", e);
            axum::Json(Vec::<product::Product>::new())
        }
    }
}

async fn refresh(State(db): State<DB>) -> &'static str {
    tokio::spawn(fetch_products(db))
        .await
        .expect("Error while fetching items")
}

async fn fetch_products(db: DB) -> &'static str {
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
