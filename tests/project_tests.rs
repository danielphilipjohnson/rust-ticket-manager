mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use ticket_manager::routes::router::create_router;
use common::{
    create_empty_request, create_json_request, create_test_pool, setup_test_issue,
    setup_test_project,
};

#[tokio::test]
async fn test_create_project_success() {
    let app = create_router(create_test_pool());

    let request = Request::builder()
        .method("POST")
        .uri("/api/projects")
        .header("Content-Type", "application/json")
        .body(Body::from(
            json!({
                "name": "New Project",
                "description": "Project Description"
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_project_invalid_data() {
    let app = create_router(create_test_pool());

    let request_body = json!({
        "description": "Missing name field"
    })
    .to_string();

    let request = Request::builder()
        .method("POST")
        .uri("/api/projects")
        .header("Content-Type", "application/json")
        .body(Body::from(request_body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

#[tokio::test]
async fn test_get_projects() {
    let app = create_router(create_test_pool());

    let request = Request::builder()
        .method("GET")
        .uri("/api/projects")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_project() {
    let app = create_router(create_test_pool());

    let project_id = setup_test_project(&create_test_pool()).await;

    let request_body = json!({
        "name": "Updated Name",
        "description": "Updated Description"
    })
    .to_string();

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/projects/{}", project_id))
        .header("Content-Type", "application/json")
        .body(Body::from(request_body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_delete_project() {
    let app = create_router(create_test_pool());

    let request = Request::builder()
        .method("DELETE")
        .uri("/api/projects/1")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert!(response.status().is_success());
}
