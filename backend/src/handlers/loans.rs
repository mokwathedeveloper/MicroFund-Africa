use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::Loan;

#[derive(Deserialize)]
pub struct CreateLoanRequest {
    pub amount: f64,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RepayLoanRequest {
    pub loan_id: Uuid,
}

pub async fn create_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<CreateLoanRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(&req) {
        Ok(id) => id,
        Err(res) => return res,
    };

    // In a real hackathon demo, we would also trigger a Solana transaction here
    // for transparency. For now, we log the intent.
    log::info!("User {} requesting loan of {}", user_id, form.amount);

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
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(e) => {
            log::error!("Failed to create loan: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create loan")
        }
    }
}

pub async fn repay_loan(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<RepayLoanRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(&req) {
        Ok(id) => id,
        Err(res) => return res,
    };

    let result = sqlx::query!(
        "UPDATE loans SET status = 'repaid', repaid_at = NOW() WHERE id = $1 AND user_id = $2 RETURNING id",
        form.loan_id,
        user_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(_)) => HttpResponse::Ok().body("Loan repaid successfully"),
        Ok(None) => HttpResponse::NotFound().body("Loan not found or unauthorized"),
        Err(e) => {
            log::error!("Failed to repay loan: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to repay loan")
        }
    }
}

pub async fn get_loans(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match get_user_id_from_req(&req) {
        Ok(id) => id,
        Err(res) => return res,
    };

    let result = sqlx::query_as!(
        Loan,
        "SELECT id, user_id, amount::float8 as amount, status, description, created_at, repaid_at FROM loans WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(loans) => HttpResponse::Ok().json(loans),
        Err(e) => {
            log::error!("Failed to fetch loans: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch loans")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

fn get_user_id_from_req(req: &HttpRequest) -> Result<Uuid, HttpResponse> {
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
                Err(_) => return Err(HttpResponse::Unauthorized().body("Invalid token")),
            }
        }
    }
    Err(HttpResponse::Unauthorized().body("Missing or invalid authorization header"))
}
