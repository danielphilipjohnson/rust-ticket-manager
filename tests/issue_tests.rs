mod common;
use axum::body::Body;
use hyper::{Request};


use axum::http::StatusCode;
use common::{
    create_empty_request, create_json_request, create_test_pool, setup_test_issue,
    setup_test_project,
};
use serde_json::json;
use ticket_manager::routes::router::create_router;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_issue() {
    let app = create_router(create_test_pool());

    let project_id = setup_test_project(&create_test_pool()).await;

    let request_body = json!({
        "title": "Test Issue",
        "description": "This is a test issue",
        "created_by": "test_user",
        "assigned_to": "test_assignee",
        "status": "open"
    })
    .to_string();

    let request = Request::builder()
        .method("POST")
        .uri(&format!("/api/projects/{}/issues", project_id))
        .header("Content-Type", "application/json")
        .body(Body::from(request_body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_project_issues() {
    let pool = create_test_pool();
    let app = create_router(pool.clone());

    let project_id = setup_test_project(&pool).await;
    let _issue_id = setup_test_issue(&pool, project_id).await;

    let request = create_empty_request("GET", &format!("/api/projects/{}/issues", project_id));

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_issue() {
    let app = create_router(create_test_pool());

    let project_id = setup_test_project(&create_test_pool()).await;

    let issue_id = setup_test_issue(&create_test_pool(), project_id).await;

    let request_body = json!({
        "title": "Updated Title",
        "description": "Updated Description",
        "status": "in_progress"
    })
    .to_string();

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/projects/{}/issues/{}", project_id, issue_id))
        .header("Content-Type", "application/json")
        .body(Body::from(request_body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}


#[tokio::test]
async fn test_delete_issue() {
    let app = create_router(create_test_pool());

    let project_id = setup_test_project(&create_test_pool()).await;

    let issue_id = setup_test_issue(&create_test_pool(), project_id).await;

    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/projects/{}/issues/{}", project_id, issue_id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    
    assert!(response.status() == StatusCode::NO_CONTENT || response.status() == StatusCode::OK);
}


#[tokio::test]
async fn test_get_nonexistent_project_issues() {
    let app = create_router(create_test_pool());

    let request = create_empty_request("GET", "/api/projects/999/issues");

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_issue_invalid_data() {
    let pool = create_test_pool();
    let app = create_router(pool.clone());

    let project_id = setup_test_project(&pool).await;

    let request = create_json_request(
        "POST",
        &format!("/api/projects/{}/issues", project_id),
        json!({
            "description": "Missing required fields"
        }),
    );

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
