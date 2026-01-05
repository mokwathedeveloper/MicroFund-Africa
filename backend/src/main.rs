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
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
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
