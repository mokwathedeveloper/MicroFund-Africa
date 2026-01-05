use actix_web::{web, HttpResponse, HttpRequest};
use serde::{Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::middleware::AppError;
use crate::handlers::loans::get_user_id_from_req;
use crate::models::Savings;

use crate::services::mpesa::MpesaService;

#[derive(Deserialize)]
pub struct CreateSavingsRequest {
    pub goal_name: String,
}

#[derive(Deserialize)]
pub struct DepositRequest {
    pub amount: f64,
    pub phone_number: Option<String>,
}

pub async fn get_savings(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query_as!(
        Savings,
        "SELECT id, user_id, amount::float8 as amount, goal_name, created_at, updated_at FROM savings WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(savings) => Ok(HttpResponse::Ok().json(savings)),
        Err(e) => {
            log::error!("Failed to fetch savings: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn create_savings(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<CreateSavingsRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id_from_req(&req)?;

    let result = sqlx::query!(
        "INSERT INTO savings (user_id, goal_name) VALUES ($1, $2) RETURNING id",
        user_id,
        form.goal_name
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record.id)),
        Err(e) => {
            log::error!("Failed to create savings goal: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn deposit(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    savings_id: web::Path<Uuid>,
    form: web::Json<DepositRequest>,
) -> Result<HttpResponse, AppError> {
    let _user_id = get_user_id_from_req(&req)?;

    // Simulate M-Pesa Payment if phone number is provided
    if let Some(phone) = &form.phone_number {
        MpesaService::initiate_stk_push(phone, form.amount)
            .await
            .map_err(|_| AppError::InternalServerError)?;
    }

    let mut tx = pool.begin().await.map_err(|_| AppError::InternalServerError)?;

    sqlx::query!(
        "UPDATE savings SET amount = amount + $1, updated_at = NOW() WHERE id = $2",
        form.amount as f32,
        *savings_id
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    sqlx::query!(
        "INSERT INTO savings_transactions (savings_id, amount, transaction_type) VALUES ($1, $2, $3)",
        *savings_id,
        form.amount as f32,
        "deposit"
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| AppError::InternalServerError)?;

    tx.commit().await.map_err(|_| AppError::InternalServerError)?;

    Ok(HttpResponse::Ok().body("Deposit successful"))
}
