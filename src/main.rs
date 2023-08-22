use axum::Router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod models;
mod routes;
mod schema;
mod services;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new().allow_origin(Any);

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new()
        .merge(services::backend(app_state.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 80));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
