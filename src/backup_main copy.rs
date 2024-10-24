mod services;
mod model;
mod schema;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started successfully!");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    dotenv().ok();
    env_logger::init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to DB established");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the database: {:?}", error);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
            .wrap(Logger::default()) // <- aqui
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}