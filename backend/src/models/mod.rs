use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Loan {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lender_id: Option<Uuid>,
    pub amount: f64,
    pub status: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub repaid_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Savings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub goal_name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SavingsTransaction {
    pub id: Uuid,
    pub savings_id: Uuid,
    pub amount: f64,
    pub transaction_type: String,
    pub created_at: Option<DateTime<Utc>>,
}
