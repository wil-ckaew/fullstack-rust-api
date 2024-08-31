use actix_web::{test, App, HttpResponse, web::Json};
use uuid::Uuid;
use crate::{services::tasks, schema::CreateTaskSchema, model::TaskModel};

#[actix_rt::test]
async fn test_create_task() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(tasks::config_tasks)
    ).await;

    // Act
    let new_task = CreateTaskSchema {
        title: "Test Task".to_string(),
        content: "This is a test task".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    assert!(response_text.contains("success"));
}

#[actix_rt::test]
async fn test_get_all_tasks() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(tasks::config_tasks)
    ).await;

    // Act
    let req = test::TestRequest::get()
        .uri("/tasks")
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    assert!(response_text.contains("success"));
}

#[actix_rt::test]
async fn test_get_task_by_id() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(tasks::config_tasks)
    ).await;

    // Insert a task first
    let new_task = CreateTaskSchema {
        title: "Test Task".to_string(),
        content: "This is a test task".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    let task_id = response_text.split('"').nth(7).unwrap();

    // Act
    let req = test::TestRequest::get()
        .uri(&format!("/tasks/{}", task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    assert!(response_text.contains("success"));
}

#[actix_rt::test]
async fn test_update_task_by_id() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(tasks::config_tasks)
    ).await;

    // Insert a task first
    let new_task = CreateTaskSchema {
        title: "Test Task".to_string(),
        content: "This is a test task".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    let task_id = response_text.split('"').nth(7).unwrap();

    // Update the task
    let update_task = crate::schema::UpdateTaskSchema {
        title: Some("Updated Task".to_string()),
        content: None,
    };

    let req = test::TestRequest::patch()
        .uri(&format!("/tasks/{}", task_id))
        .set_json(&update_task)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    assert!(response_text.contains("success"));
}

#[actix_rt::test]
async fn test_delete_task_by_id() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(tasks::config_tasks)
    ).await;

    // Insert a task first
    let new_task = CreateTaskSchema {
        title: "Test Task".to_string(),
        content: "This is a test task".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/tasks")
        .set_json(&new_task)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    let task_id = response_text.split('"').nth(7).unwrap();

    // Delete the task
    let req = test::TestRequest::delete()
        .uri(&format!("/tasks/{}", task_id))
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response_text = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(response_text, "");
}
