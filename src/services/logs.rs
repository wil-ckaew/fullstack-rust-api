use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::LogModel,
    schema::{CreateLogSchema, UpdateLogSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/logs")]
async fn create_log(
    body: Json<CreateLogSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO logs (user_id, action, description)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, action, description, timestamp
    "#;

    match sqlx::query_as::<_, LogModel>(query)
        .bind(&body.user_id)
        .bind(&body.action)
        .bind(&body.description)
        .fetch_one(&data.db)
        .await
    {
        Ok(log) => {
            let response = json!({
                "status": "success",
                "log": {
                    "id": log.id,
                    "user_id": log.user_id,
                    "action": log.action,
                    "description": log.description,
                    "timestamp": log.timestamp
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create log: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/logs")]
pub async fn get_all_logs(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        LogModel,
        "SELECT * FROM logs ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(logs) => {
            let response = json!({
                "status": "success",
                "logs": logs
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get logs: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/logs/{id}")]
async fn get_log_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let log_id = path.into_inner();

    match sqlx::query_as!(
        LogModel,
        "SELECT * FROM logs WHERE id = $1",
        log_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(log) => {
            let response = json!({
                "status": "success",
                "log": log
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get log: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[patch("/logs/{id}")]
async fn update_log_by_id(
    path: Path<Uuid>,
    body: Json<UpdateLogSchema>,
    data: Data<AppState>
) -> impl Responder {
    let log_id = path.into_inner();

    match sqlx::query_as!(LogModel, "SELECT * FROM logs WHERE id = $1", log_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(log) => {
            let update_result = sqlx::query_as!(
                LogModel,
                "UPDATE logs SET user_id = COALESCE($1, user_id), action = COALESCE($2, action), description = COALESCE($3, description) WHERE id = $4 RETURNING *",
                body.user_id.as_ref(),
                body.action.as_ref(),
                body.description.as_ref(),
                log_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_log) => {
                    let response = json!({
                        "status": "success",
                        "log": updated_log
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update log: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Log not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/logs/{id}")]
async fn delete_log_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let log_id = path.into_inner();

    match sqlx::query!("DELETE FROM logs WHERE id = $1", log_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete log: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para tarefas
pub fn config_logs(conf: &mut ServiceConfig) {
    conf.service(create_log)
       .service(get_all_logs)
       .service(get_log_by_id)
       .service(update_log_by_id)
       .service(delete_log_by_id);
}