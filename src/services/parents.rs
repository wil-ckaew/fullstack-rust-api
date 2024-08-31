use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::ParentModel,
    schema::{CreateParentSchema, UpdateParentSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/parents")]
async fn create_parent(
    body: Json<CreateParentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO parents (name, email, phone)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, phone, created_at
    "#;

    match sqlx::query_as::<_, ParentModel>(query)
        .bind(&body.name)
        .bind(&body.email)
        .bind(&body.phone)
        .fetch_one(&data.db)
        .await
    {
        Ok(parent) => {
            let response = json!({
                "status": "success",
                "parent": {
                    "id": parent.id,
                    "name": parent.name,
                    "email": parent.email,
                    "phone": parent.phone,
                    "created_at": parent.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create parent: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/parents")]
pub async fn get_all_parents(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        ParentModel,
        "SELECT * FROM parents ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(parents) => {
            let response = json!({
                "status": "success",
                "parents": parents
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get parents: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/parents/{id}")]
async fn get_parent_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let parent_id = path.into_inner();

    match sqlx::query_as!(
        ParentModel,
        "SELECT * FROM parents WHERE id = $1",
        parent_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(parent) => {
            let response = json!({
                "status": "success",
                "parent": parent
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get parent: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[patch("/parents/{id}")]
async fn update_parent_by_id(
    path: Path<Uuid>,
    body: Json<UpdateParentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let parent_id = path.into_inner();

    match sqlx::query_as!(ParentModel, "SELECT * FROM parents WHERE id = $1", parent_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(parent) => {
            let update_result = sqlx::query_as!(
                ParentModel,
                "UPDATE parents SET name = COALESCE($1, name), email = COALESCE($2, email), phone = COALESCE($3, phone) WHERE id = $4 RETURNING *",
                body.name.as_ref(),
                body.email.as_ref(),
                body.phone.as_ref(),
                parent_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_parent) => {
                    let response = json!({
                        "status": "success",
                        "parent": updated_parent
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update parent: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Parent not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/parents/{id}")]
async fn delete_parent_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let parent_id = path.into_inner();

    match sqlx::query!("DELETE FROM parents WHERE id = $1", parent_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete parent: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para tarefas
pub fn config_parents(conf: &mut ServiceConfig) {
    conf.service(create_parent)
       .service(get_all_parents)
       .service(get_parent_by_id)
       .service(update_parent_by_id)
       .service(delete_parent_by_id);
}