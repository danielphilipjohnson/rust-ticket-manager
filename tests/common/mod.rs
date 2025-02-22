use axum::{body::Body, http::Request};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::PgConnection;
use diesel::RunQueryDsl;
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use ticket_manager::{
    models::{Issue, Project},
    routes::router::DbPool,
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn create_test_pool() -> DbPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}

pub async fn setup_test_project(pool: &DbPool) -> i32 {
    cleanup_test_data(pool).await;

    let mut conn = pool.get().expect("Failed to get DB connection");
    let unique_id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let project_name = format!("Test Project {}", unique_id);

    let project = Project::create(&mut conn, &project_name, "Test Description")
        .expect("Failed to create test project");

    project.id
}

pub async fn setup_test_issue(pool: &DbPool, project_id: i32) -> i32 {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let unique_id = COUNTER.fetch_add(1, Ordering::SeqCst);

    let issue = Issue::create(
        &mut conn,
        project_id,
        &format!("Test Issue {}", unique_id),
        "Test Description",
        "test_user",
        None,
        "open",
    )
    .expect("Failed to create test issue");

    issue.id
}

pub fn create_json_request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

pub fn create_empty_request(method: &str, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

pub async fn cleanup_test_data(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get DB connection");

    sql_query("TRUNCATE TABLE projects RESTART IDENTITY CASCADE")
        .execute(&mut conn)
        .expect("Failed to truncate test data");
}
