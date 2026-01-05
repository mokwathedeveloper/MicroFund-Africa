use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;
mod db;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logger with 'info' level as default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Connect to PostgreSQL database using the DATABASE_URL environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    log::info!("MicroFund Africa Backend starting at http://127.0.0.1:8080");

    // Initialize and run the Actix-web server
    HttpServer::new(move || {
        App::new()
            // Inject the DB pool into the application state
            .app_data(web::Data::new(pool.clone()))
            // Enable default request logging
            .wrap(Logger::default())
            // Register all API routes under the /api scope
            .service(
                web::scope("/api")
                    .configure(handlers::config)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
mod tests;
