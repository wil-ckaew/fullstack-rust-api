use actix_web::web::{self, ServiceConfig};

pub mod tasks;
pub mod documents;

pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        web::scope("/api")
            .configure(tasks::config)
            .configure(documents::config)
    );
}
