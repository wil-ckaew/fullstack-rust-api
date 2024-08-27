mod services; // Importa o módulo services
mod model;
mod schema;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializa as variáveis de ambiente
    dotenv().ok();
    env_logger::init();

    // Configura o log padrão, se não estiver definido
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    // Obtém a URL do banco de dados das variáveis de ambiente
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Cria o pool de conexões para o PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    println!("Server started successfully!");
    println!("Connection to DB established");

    // Configura e inicia o servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config) // Configura as rotas definidas em services
            .wrap(Logger::default()) // Configura o middleware de logging
    })
    .bind("127.0.0.1:8080")? // Define o endereço e a porta
    .run()
    .await
}
