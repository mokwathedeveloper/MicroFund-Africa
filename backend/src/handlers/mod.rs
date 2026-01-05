use actix_web::{web, HttpResponse};
use serde::Serialize;
use sqlx::PgPool;
use crate::middleware::AppError;

pub mod auth;
pub mod loans;
pub mod savings;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/profile", web::get().to(auth::get_profile))
    )
    .service(
        web::scope("/loans")
            .route("", web::post().to(loans::create_loan))
            .route("", web::get().to(loans::get_loans))
            .route("/marketplace", web::get().to(loans::get_marketplace))
            .route("/{id}/fund", web::post().to(loans::fund_loan))
            .route("/repay", web::post().to(loans::repay_loan))
    )
    .service(
        web::scope("/savings")
            .route("", web::get().to(savings::get_savings))
            .route("", web::post().to(savings::create_savings))
            .route("/{id}/deposit", web::post().to(savings::deposit))
    )
    .route("/stats", web::get().to(get_platform_stats))
    .route("/ledger", web::get().to(get_live_ledger))
    .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }));
}

async fn get_live_ledger(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let ledger: Vec<crate::models::PlatformTransaction> = sqlx::query_as(
        "SELECT id, activity_type, description, amount::float8 as amount, signature, created_at FROM platform_transactions ORDER BY created_at DESC LIMIT 50"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(ledger))
}

#[derive(Serialize)]
struct PlatformStats {
    total_users: i64,
    total_loans_value: f64,
    total_savings_value: f64,
    active_p2p_deals: i64,
}

async fn get_platform_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let users_count: (i64,) = sqlx::query_as("SELECT count(*) FROM users")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let loans_value: (Option<f64>,) = sqlx::query_as("SELECT sum(amount)::float8 FROM loans")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let savings_value: (Option<f64>,) = sqlx::query_as("SELECT sum(amount)::float8 FROM savings")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let marketplace_count: (i64,) = sqlx::query_as("SELECT count(*) FROM loans WHERE status = 'pending'")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(PlatformStats {
        total_users: users_count.0,
        total_loans_value: loans_value.0.unwrap_or(0.0),
        total_savings_value: savings_value.0.unwrap_or(0.0),
        active_p2p_deals: marketplace_count.0,
    }))
}
