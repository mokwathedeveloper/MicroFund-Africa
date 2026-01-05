use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::Loan;
use crate::middleware::AppError;
use crate::services::blockchain::BlockchainService;

#[derive(Deserialize)]
pub struct CreateLoanRequest {
    pub amount: f64,
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

    let result = sqlx::query_as!(
        MarketplaceLoan,
        "SELECT l.id, l.user_id, u.username as borrower_username, l.amount::float8 as amount, l.description, l.created_at 
         FROM loans l 
         JOIN users u ON l.user_id = u.id 
         WHERE l.status = 'pending' AND l.user_id != $1",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(loans) => Ok(HttpResponse::Ok().json(loans)),
        Err(e) => {
            log::error!("Failed to fetch marketplace: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn fund_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    loan_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query!(
        "UPDATE loans SET lender_id = $1, status = 'approved' WHERE id = $2 AND status = 'pending' AND user_id != $1 RETURNING id",
        user_id,
        *loan_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(record)) => {
            BlockchainService::log_loan_repayment(record.id, "FUNDING_SIG").await.ok();
            Ok(HttpResponse::Ok().body("Loan funded successfully"))
        },
        Ok(None) => Err(AppError::BadRequest("Loan not available for funding".to_string())),
        Err(e) => {
            log::error!("Failed to fund loan: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn create_loan(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    form: web::Json<CreateLoanRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    // Log the transaction intent to the blockchain simulation
    let _ = BlockchainService::log_loan_initialization(
        Uuid::new_v4(), 
        form.amount, 
        &user_id.to_string()
    ).await.map_err(|_| AppError::InternalServerError)?;

    let result = sqlx::query!(
        "INSERT INTO loans (user_id, amount, description, status) VALUES ($1, $2, $3, $4) RETURNING id",
        user_id,
        form.amount as f32,
        form.description,
        "pending"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record.id)),
        Err(e) => {
            log::error!("Failed to create loan: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn repay_loan(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    form: web::Json<RepayLoanRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query!(
        "UPDATE loans SET status = 'repaid', repaid_at = NOW() WHERE id = $1 AND user_id = $2 RETURNING id",
        form.loan_id,
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(record)) => {
            // Log repayment to blockchain
            let _ = BlockchainService::log_loan_repayment(record.id, "SIMULATED_SIG")
                .await
                .map_err(|_| AppError::InternalServerError)?;
                
            Ok(HttpResponse::Ok().body("Loan repaid successfully"))
        },
        Ok(None) => Err(AppError::NotFound),
        Err(e) => {
            log::error!("Failed to repay loan: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn get_loans(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query_as!(
        Loan,
        "SELECT id, user_id, lender_id, amount::float8 as amount, status, description, created_at, repaid_at FROM loans WHERE user_id = $1 OR lender_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(loans) => Ok(HttpResponse::Ok().json(loans)),
        Err(e) => {
            log::error!("Failed to fetch loans: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

fn get_user_id_from_req(req: &HttpRequest) -> Result<Uuid, AppError> {
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