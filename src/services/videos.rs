use actix_web::{
    get, post, delete, patch, web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder
};
use actix_files::Files;
use actix_multipart::Multipart;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use std::path::PathBuf;

use crate::{
    model::{VideoModel, PhotoModel, StudentModel},
    schema::{CreateVideoSchema, UpdateVideoSchema, FilterOptions, CreatePhotoSchema, UpdatePhotoSchema},
    AppState
};

// Função para criar um novo vídeo
#[post("/videos")]
async fn create_video(
    body: Json<CreateVideoSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO videos (student_id, filename, description)
        VALUES ($1, $2, $3)
        RETURNING id, student_id, filename, description, created_at
    "#;

    match sqlx::query_as::<_, VideoModel>(query)
        .bind(&body.student_id)
        .bind(&body.filename)
        .bind(&body.description)
        .fetch_one(&data.db)
        .await
    {
        Ok(video) => {
            let response = json!( {
                "status": "success",
                "video": {
                    "id": video.id,
                    "student_id": video.student_id,
                    "filename": video.filename,
                    "description": video.description,
                    "created_at": video.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to create video: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Função para obter todos os vídeos
#[get("/videos")]
pub async fn get_all_videos(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        VideoModel,
        "SELECT * FROM videos ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(videos) => {
            let response = json!( {
                "status": "success",
                "videos": videos
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to get videos: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Função para obter um vídeo por ID
#[get("/videos/{id}")]
async fn get_video_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let video_id = path.into_inner();

    match sqlx::query_as!(
        VideoModel,
        "SELECT * FROM videos WHERE id = $1",
        video_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(video) => {
            let response = json!( {
                "status": "success",
                "video": video
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to get video: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Função para atualizar um vídeo
#[patch("/videos/{id}")]
async fn update_video_by_id(
    path: Path<Uuid>,
    body: Json<UpdateVideoSchema>,
    data: Data<AppState>
) -> impl Responder {
    let video_id = path.into_inner();

    match sqlx::query_as!(
        VideoModel,
        "SELECT * FROM videos WHERE id = $1",
        video_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(_) => {
            let update_result = sqlx::query_as!(
                VideoModel,
                "UPDATE videos SET student_id = COALESCE($1, student_id), filename = COALESCE($2, filename), description = COALESCE($3, description) WHERE id = $4 RETURNING *",
                body.student_id.as_ref(),
                body.filename.as_ref(),
                body.description.as_ref(),
                video_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_video) => {
                    let response = json!( {
                        "status": "success",
                        "video": updated_video
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!( {
                        "status": "error",
                        "message": format!("Failed to update video: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Video not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

// Função para deletar um vídeo por ID
#[delete("/videos/{id}")]
async fn delete_video_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let video_id = path.into_inner();

    match sqlx::query!("DELETE FROM videos WHERE id = $1", video_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to delete video: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Função para upload de vídeo
#[post("/upload-video")]
async fn upload_video(
    data: Data<AppState>,
    mut payload: Multipart
) -> impl Responder {
    while let Some(field) = payload.next().await {
        match field {
            Ok(mut field) => {
                // Get the filename from the field
                let filename = field.content_disposition()
                    .get_filename()
                    .map_or("temp.mp4".to_string(), |f| f.to_string());
                let filepath = format!("./uploads/{}", filename);
                println!("Saving file to: {}", filepath);

                // Create a file in the uploads directory
                let mut file = match tokio::fs::File::create(&filepath).await {
                    Ok(file) => file,
                    Err(e) => return HttpResponse::InternalServerError().json({
                        let message = format!("Failed to create file: {:?}", e);
                        println!("{}", message);
                        json!({ "status": "error", "message": message })
                    }),
                };

                // Write the chunks into the file
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => {
                            if let Err(e) = file.write_all(&data).await {
                                return HttpResponse::InternalServerError().json({
                                    let message = format!("Failed to write to file: {:?}", e);
                                    println!("{}", message);
                                    json!({ "status": "error", "message": message })
                                });
                            }
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError().json({
                                let message = format!("Failed to read chunk: {:?}", e);
                                println!("{}", message);
                                json!({ "status": "error", "message": message })
                            });
                        }
                    }
                }
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json({
                    let message = format!("Failed to process file: {:?}", e);
                    println!("{}", message);
                    json!({ "status": "error", "message": message })
                });
            }
        }
    }

    HttpResponse::Ok().json(json!( {
        "status": "success",
        "message": "File uploaded successfully"
    }))
}


// Função para obter todos os alunos
#[get("/students")]
async fn get_students(data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(StudentModel, "SELECT * FROM students")
        .fetch_all(&data.db)
        .await
    {
        Ok(students) => {
            let response = json!( {
                "status": "success",
                "students": students
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!( {
                "status": "error",
                "message": format!("Failed to get students: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Configuração das rotas para vídeos e alunos
pub fn config_videos(conf: &mut ServiceConfig) {
    conf.service(create_video)
       .service(get_all_videos)
       .service(get_video_by_id)
       .service(update_video_by_id)
       .service(delete_video_by_id)
       .service(upload_video) // Adicionando o serviço de upload de vídeo
       .service(Files::new("/static", "./static").show_files_listing())
       .service(get_students);
}
