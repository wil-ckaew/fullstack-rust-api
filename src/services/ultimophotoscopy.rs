use actix_web::{
    get, post, delete, patch, web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder
};
use actix_files::Files; // Para servir arquivos estáticos
use actix_multipart::Multipart; // Importação do Multipart
use futures_util::StreamExt; // Para a extensão de Stream
use tokio::io::AsyncWriteExt; // Importar AsyncWriteExt para usar write_all
use std::fs; // Para operações de criação de arquivos (não usado neste contexto assíncrono)
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{
    model::PhotoModel,
    schema::{CreatePhotoSchema, UpdatePhotoSchema, FilterOptions},
    AppState
};

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

    match sqlx::query_as!(
        PhotoModel,
        "SELECT * FROM photos WHERE id = $1",
        photo_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(_) => {
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


#[post("/upload")]
async fn upload_image(
    mut payload: Multipart
) -> impl Responder {
    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => { // Tornar a variável `field` mutável aqui
                let filename = field
                    .content_disposition()
                    .get_filename()
                    .map_or("temp".to_string(), |f| f.to_string());

                let filepath = format!("./uploads/{}", filename);

                // Criar o arquivo de forma assíncrona
                let file_result = match tokio::fs::File::create(&filepath).await {
                    Ok(f) => f,
                    Err(e) => return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": format!("Failed to create file: {:?}", e)
                    })),
                };

                let mut file = tokio::io::BufWriter::new(file_result);

                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(d) => d,
                        Err(e) => return HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": format!("Failed to read chunk: {:?}", e)
                        })),
                    };

                    // Escrever dados no arquivo
                    if let Err(e) = file.write_all(&data).await {
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": format!("Failed to write data: {:?}", e)
                        }));
                    }
                }
            }
            Err(e) => return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Failed to process file: {:?}", e)
            })),
        }
    }

    HttpResponse::Ok().finish()
}

// Configuração das rotas
pub fn config_photos(conf: &mut ServiceConfig) {
    conf.service(create_photo)
       .service(get_all_photos)
       .service(get_photo_by_id)
       .service(update_photo_by_id)
       .service(delete_photo_by_id)
       .service(Files::new("/uploads", "./uploads").show_files_listing())
       .service(upload_image);
}
