use actix_web::{test, App, web, HttpResponse, HttpServer};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use your_crate::{services::documents::config_documents, AppState}; // Ajuste conforme o nome do seu crate

async fn setup_test_app() -> App<impl actix_service::ServiceFactory> {
    // Crie uma conexão com o banco de dados de teste
    let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();
    let app_state = web::Data::new(AppState { db: pool });

    // Inicialize o servidor de testes com a configuração de rotas
    test::init_service(App::new().app_data(app_state.clone()).configure(config_documents)).await
}

#[actix_rt::test]
async fn test_health_checker() {
    let app = setup_test_app().await;

    let req = test::TestRequest::get().uri("/healthchecker").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "success");
    assert_eq!(body["message"], "Health check: API is up and running smoothly.");
}

#[actix_rt::test]
async fn test_create_document() {
    let app = setup_test_app().await;

    let new_doc = json!({
        "user_id": Uuid::new_v4().to_string(),
        "doc_type": "pdf"
    });

    let req = test::TestRequest::post()
        .uri("/documents")
        .set_json(&new_doc)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "success");
    assert!(body["document"]["id"].is_string());
    assert_eq!(body["document"]["doc_type"], "pdf");
}

#[actix_rt::test]
async fn test_get_all_documents() {
    let app = setup_test_app().await;

    let req = test::TestRequest::get().uri("/documents").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "success");
    assert!(body["documents"].is_array());
}

#[actix_rt::test]
async fn test_get_document_by_id() {
    let app = setup_test_app().await;

    // Assumindo que você já tem um documento de teste no banco de dados
    let document_id = Uuid::new_v4(); // Substitua por um ID real se necessário

    let req = test::TestRequest::get().uri(&format!("/documents/{}", document_id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "success");
    assert_eq!(body["document"]["id"], document_id.to_string());
}

#[actix_rt::test]
async fn test_delete_document_by_id() {
    let app = setup_test_app().await;

    // Assumindo que você já tem um documento de teste no banco de dados
    let document_id = Uuid::new_v4(); // Substitua por um ID real se necessário

    let req = test::TestRequest::delete().uri(&format!("/documents/{}", document_id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), HttpResponse::NoContent().status());

    // Tente buscar o documento para garantir que foi excluído
    let req = test::TestRequest::get().uri(&format!("/documents/{}", document_id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), HttpResponse::NotFound().status());
}

#[actix_rt::test]
async fn test_update_document_by_id() {
    let app = setup_test_app().await;

    // Assumindo que você já tem um documento de teste no banco de dados
    let document_id = Uuid::new_v4(); // Substitua por um ID real se necessário

    let update_data = json!({
        "user_id": Uuid::new_v4().to_string(),
        "doc_type": "updated_pdf"
    });

    let req = test::TestRequest::patch()
        .uri(&format!("/documents/{}", document_id))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "success");
    assert_eq!(body["document"]["doc_type"], "updated_pdf");
}
