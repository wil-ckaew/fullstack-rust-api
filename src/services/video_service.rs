use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures_util::StreamExt as _;
use std::io::Write;

async fn upload_video(mut payload: Multipart) -> impl Responder {
    while let Some(field) = payload.next().await {
        let mut field = field.unwrap();
        let filename = field
            .file_name()
            .unwrap_or("unknown".to_string());

        // Especifique o caminho onde os arquivos serão armazenados
        let filepath = format!("./uploads/{}", filename);
        
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).unwrap();
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/videos").route(web::post().to(upload_video)));
}
