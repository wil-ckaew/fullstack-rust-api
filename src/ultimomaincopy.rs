// src/main.rs
mod services;
mod model;
mod schema;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started successfully!");

    // Configure logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    // Retrieve database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create the database connection pool
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to DB established");
            pool
        }
        Err(error) => {
            eprintln!("Failed to connect to the database: {:?}", error);
            std::process::exit(1);
        }
    };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() })) // Share the database pool across handlers
            .configure(services::config) // Register routes and services
            .wrap(Logger::default()) // Enable request logging
            .wrap(Cors::default()
                .allow_any_origin() // Allow requests from any origin
                .allow_any_method() // Allow any HTTP method
                .allow_any_header() // Allow any headers
            )
    })
    .bind("127.0.0.1:8080")? // Bind the server to port 8080
    .run()
    .await
}
