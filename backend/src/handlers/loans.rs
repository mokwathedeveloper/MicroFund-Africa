use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use chrono::{DateTime, Utc};
use crate::models::Loan;
use crate::middleware::AppError;
use crate::services::blockchain::BlockchainService;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateLoanRequest {
    #[validate(range(min = 1.0, max = 5000.0, message = "Loan amount must be between $1 and $5000"))]
    pub amount: f64,
    #[validate(length(min = 3, message = "Please provide a valid reason"))]
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RepayLoanRequest {
    pub loan_id: Uuid,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct MarketplaceLoan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub borrower_username: String,
    pub amount: f64,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub async fn get_marketplace(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let loans: Vec<MarketplaceLoan> = sqlx::query_as(
        "SELECT l.id, l.user_id, u.username as borrower_username, l.amount::float8 as amount, l.description, l.created_at 
         FROM loans l 
         JOIN users u ON l.user_id = u.id 
         WHERE l.status = 'pending' AND l.user_id != $1"
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch marketplace: {:?}", e);
        AppError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(loans))
}

pub async fn fund_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    loan_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query(
        "UPDATE loans SET lender_id = $1, status = 'approved' WHERE id = $2 AND status = 'pending' AND user_id != $1 RETURNING id"
    )
    .bind(user_id)
    .bind(*loan_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to fund loan: {:?}", e);
        AppError::InternalServerError
    })?;

    match result {
        Some(_) => {
            BlockchainService::log_loan_repayment(*loan_id, "FUNDING_SIG").await.ok();
            Ok(HttpResponse::Ok().body("Loan funded successfully"))
        },
        None => Err(AppError::BadRequest("Loan not available for funding".to_string())),
    }
}

pub async fn create_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<CreateLoanRequest>,
) -> Result<HttpResponse, AppError> {
    form.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let user_id = get_user_id_from_req(&req)?;

    // INNOVATION: Reputation-based Dynamic Limits
    let reputation: (Option<i32>,) = sqlx::query_as("SELECT reputation_score FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let max_limit = (reputation.0.unwrap_or(100) as f64) * 2.0;
    
    if form.amount > max_limit {
        return Err(AppError::BadRequest(format!(
            "Your Trust Score restricts loans to ${:.2}. Repay more loans to increase your limit!", 
            max_limit
        )));
    }

    tracing::info!("User {} creating loan of ${}", user_id, form.amount);

    let _ = BlockchainService::log_to_ledger(
        pool.get_ref(),
        "LOAN_REQUEST",
        &format!("Loan for: {}", form.description.clone().unwrap_or_default()),
        form.amount
    ).await;

    let result = sqlx::query(
        "INSERT INTO loans (user_id, amount, description, status) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(user_id)
    .bind(form.amount as f32)
    .bind(&form.description)
    .bind("pending")
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create loan: {:?}", e);
        AppError::InternalServerError
    })?;

    let id: Uuid = sqlx::Row::get(&result, "id");
    Ok(HttpResponse::Ok().json(id))
}

pub async fn repay_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<RepayLoanRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query(
        "UPDATE loans SET status = 'repaid', repaid_at = NOW() WHERE id = $1 AND user_id = $2 RETURNING id"
    )
    .bind(form.loan_id)
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to repay loan: {:?}", e);
        AppError::InternalServerError
    })?;

    match result {
        Some(_) => {
            BlockchainService::log_to_ledger(
                pool.get_ref(),
                "REPAYMENT",
                &format!("Loan {} repaid", form.loan_id),
                0.0 // Amount could be fetched if needed
            ).await.ok();

            // Increase user reputation score
            let _ = sqlx::query("UPDATE users SET reputation_score = reputation_score + 10 WHERE id = $1")
                .bind(user_id)
                .execute(pool.get_ref())
                .await;
                
            Ok(HttpResponse::Ok().body("Loan repaid successfully"))
        },
        None => Err(AppError::NotFound),
    }
}

pub async fn get_loans(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let loans: Vec<Loan> = sqlx::query_as(
        "SELECT id, user_id, lender_id, amount::float8 as amount, status, description, created_at, repaid_at FROM loans WHERE user_id = $1 OR lender_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch loans: {:?}", e);
        AppError::InternalServerError
    })?;

    Ok(HttpResponse::Ok().json(loans))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn get_user_id_from_req(req: &HttpRequest) -> Result<Uuid, AppError> {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_str) = auth_header.and_then(|h| h.to_str().ok()) {
        if auth_str.starts_with("Bearer ") {
            let token = &auth_str[7..];
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
            
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::new(Algorithm::HS256),
            );

            match token_data {
                Ok(data) => return Ok(data.claims.sub),
                Err(_) => return Err(AppError::Unauthorized),
            }
        }
    }
    Err(AppError::Unauthorized)
}