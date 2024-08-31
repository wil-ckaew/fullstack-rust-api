pub mod tasks;
pub mod documents;
pub mod users;
pub mod students;
pub mod parents;
pub mod photos;
pub mod videos;
pub mod file_metadatas;
pub mod logs;
pub mod health;

use actix_web::web::ServiceConfig;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        actix_web::web::scope("/api")
            .configure(health::config_health)
            .configure(tasks::config_tasks)
            .configure(documents::config_documents)
            .configure(users::config_users)
            .configure(students::config_students)
            .configure(parents::config_parents)
            .configure(photos::config_photos)
            .configure(videos::config_videos)
            .configure(file_metadatas::config_file_metadatas)
            .configure(logs::config_logs)
    );
}

