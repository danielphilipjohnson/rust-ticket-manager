mod config;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
use axum::serve;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber;

use crate::routes::create_router;
pub use config::DatabaseConfig;
pub use db::create_pool;
pub use error::AppError;
pub use models::Project;

pub struct AppState {
    pub pool: PgPool,
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let conn = establish_connection();
    let state = Arc::new(AppState { pool: conn });

    let app: axum::Router = create_router(state.pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app.into_make_service()).await.unwrap();
}
