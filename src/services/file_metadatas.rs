use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::FileMetadataModel,
    schema::{CreateFileMetadataSchema, UpdateFileMetadataSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

#[post("/file_metadatas")]
async fn create_file_metadata(
    body: Json<CreateFileMetadataSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO file_metadata (user_id, file_type, filename, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, file_type, filename, description, uploaded_at
    "#;

    // Gera um nome de arquivo com base no UUID
    let filename = format!("document_{}.jpg", Uuid::new_v4());

    match sqlx::query_as::<_, FileMetadataModel>(query)
        .bind(&body.user_id)
        .bind(&body.file_type)
        .bind(&filename)
        .bind(&body.description)
        .fetch_one(&data.db) // Certifique-se de usar o pool de conexão correto
        .await
    {
        Ok(file_metadata) => {
            let response = json!({
                "status": "success",
                "file_metadata": {
                    "id": file_metadata.id,
                    "user_id": file_metadata.user_id,
                    "file_type": file_metadata.file_type,
                    "filename": file_metadata.filename,
                    "description": file_metadata.description,
                    "uploaded_at": file_metadata.uploaded_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create file_metadata: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/file_metadatas")]
pub async fn get_all_file_metadatas(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        FileMetadataModel,
        "SELECT * FROM file_metadata ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(file_metadatas) => {
            let response = json!({
                "status": "success",
                "file_metadatas": file_metadatas
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get file_metadatas: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/file_metadatas/{id}")]
async fn get_file_metadata_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let file_metadata_id = path.into_inner();

    match sqlx::query_as!(
        FileMetadataModel,
        "SELECT * FROM file_metadata WHERE id = $1",
        file_metadata_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(file_metadata) => {
            let response = json!({
                "status": "success",
                "file_metadata": file_metadata
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get file_metadata: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[patch("/file_metadatas/{id}")]
async fn update_file_metadata_by_id(
    path: Path<Uuid>,
    body: Json<UpdateFileMetadataSchema>,
    data: Data<AppState>
) -> impl Responder {
    let file_metadata_id = path.into_inner();

    match sqlx::query_as!(FileMetadataModel, "SELECT * FROM file_metadata WHERE id = $1", file_metadata_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(file_metadata) => {
            let update_result = sqlx::query_as!(
                FileMetadataModel,
                "UPDATE file_metadata SET user_id = COALESCE($1, user_id), file_type = COALESCE($2, file_type), filename = COALESCE($3, filename), description = COALESCE($4, description) WHERE id = $5 RETURNING *",
                body.user_id.as_ref(),
                body.file_type.as_ref(),
                body.filename.as_ref(),
                body.description.as_ref(),
                file_metadata_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_file_metadata) => {
                    let response = json!({
                        "status": "success",
                        "file_metadata": updated_file_metadata
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update file_metadata: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("FileMetadata not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

#[delete("/file_metadatas/{id}")]
async fn delete_file_metadata_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let file_metadata_id = path.into_inner();

    match sqlx::query!("DELETE FROM file_metadata WHERE id = $1", file_metadata_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete file_metadata: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para tarefas
pub fn config_file_metadatas(conf: &mut ServiceConfig) {
    conf.service(create_file_metadata)
       .service(get_all_file_metadatas)
       .service(get_file_metadata_by_id)
       .service(update_file_metadata_by_id)
       .service(delete_file_metadata_by_id);
}