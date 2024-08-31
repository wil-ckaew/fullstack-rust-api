use actix_web::web::ServiceConfig;

use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

// Endpoint de verificação de saúde
#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

// Configuração das rotas para verificação de saúde
pub fn config_health(conf: &mut ServiceConfig) {
    conf.service(health_checker);
}
