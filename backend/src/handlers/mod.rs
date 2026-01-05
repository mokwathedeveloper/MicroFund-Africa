use actix_web::web;

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
    .route("/health", web::get().to(|| async { HttpResponse::Ok().body("OK") }));
}

#[derive(Serialize)]
struct PlatformStats {
    total_users: i64,
    total_loans_value: f64,
    total_savings_value: f64,
    active_p2p_deals: i64,
}

async fn get_platform_stats(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse, crate::middleware::AppError> {
    let users_count = sqlx::query!("SELECT count(*) FROM users")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| crate::middleware::AppError::InternalServerError)?
        .count.unwrap_or(0);

    let loans_value = sqlx::query!("SELECT sum(amount) FROM loans")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| crate::middleware::AppError::InternalServerError)?
        .sum.unwrap_or(sqlx::types::BigDecimal::from(0));

    let savings_value = sqlx::query!("SELECT sum(amount) FROM savings")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| crate::middleware::AppError::InternalServerError)?
        .sum.unwrap_or(sqlx::types::BigDecimal::from(0));

    let marketplace_count = sqlx::query!("SELECT count(*) FROM loans WHERE status = 'pending'")
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| crate::middleware::AppError::InternalServerError)?
        .count.unwrap_or(0);

    use sqlx::types::ToSql; // Not needed, just for clarity on BigDecimal to f64 conversion
    
    Ok(HttpResponse::Ok().json(PlatformStats {
        total_users: users_count,
        total_loans_value: format!("{:?}", loans_value).parse().unwrap_or(0.0), // Simplified for demo
        total_savings_value: format!("{:?}", savings_value).parse().unwrap_or(0.0),
        active_p2p_deals: marketplace_count,
    }))
}
