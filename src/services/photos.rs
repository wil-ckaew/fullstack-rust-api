use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::PhotoModel,
    schema::{CreatePhotoSchema, UpdatePhotoSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/photos")]
async fn create_photo(
    body: Json<CreatePhotoSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO photos (student_id, filename, description)
        VALUES ($1, $2, $3)
        RETURNING id, student_id, filename, description, created_at
    "#;

    match sqlx::query_as::<_, PhotoModel>(query)
        .bind(&body.student_id)
        .bind(&body.filename)
        .bind(&body.description)
        .fetch_one(&data.db)
        .await
    {
        Ok(photo) => {
            let response = json!({
                "status": "success",
                "photo": {
                    "id": photo.id,
                    "student_id": photo.student_id,
                    "filename": photo.filename,
                    "description": photo.description,
                    "created_at": photo.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create photo: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/photos")]
pub async fn get_all_photos(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        PhotoModel,
        "SELECT * FROM photos ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(photos) => {
            let response = json!({
                "status": "success",
                "photos": photos
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get photos: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/photos/{id}")]
async fn get_photo_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let photo_id = path.into_inner();

    match sqlx::query_as!(
        PhotoModel,
        "SELECT * FROM photos WHERE id = $1",
        photo_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(photo) => {
            let response = json!({
                "status": "success",
                "photo": photo
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get photo: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[patch("/photos/{id}")]
async fn update_photo_by_id(
    path: Path<Uuid>,
    body: Json<UpdatePhotoSchema>,
    data: Data<AppState>
) -> impl Responder {
    let photo_id = path.into_inner();

    match sqlx::query_as!(PhotoModel, "SELECT * FROM photos WHERE id = $1", photo_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(photo) => {
            let update_result = sqlx::query_as!(
                PhotoModel,
                "UPDATE photos SET student_id = COALESCE($1, student_id), filename = COALESCE($2, filename), description = COALESCE($3, description) WHERE id = $4 RETURNING *",
                body.student_id.as_ref(),
                body.filename.as_ref(),
                body.description.as_ref(),
                photo_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_photo) => {
                    let response = json!({
                        "status": "success",
                        "photo": updated_photo
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update photo: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Photo not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/photos/{id}")]
async fn delete_photo_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let photo_id = path.into_inner();

    match sqlx::query!("DELETE FROM photos WHERE id = $1", photo_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete photo: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para tarefas
pub fn config_photos(conf: &mut ServiceConfig) {
    conf.service(create_photo)
       .service(get_all_photos)
       .service(get_photo_by_id)
       .service(update_photo_by_id)
       .service(delete_photo_by_id);
}