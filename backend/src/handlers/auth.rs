use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::models::User;
use crate::middleware::AppError;

use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
}

pub async fn register(
    pool: web::Data<SqlitePool>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    form.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    
    tracing::info!("Registering user: {}", form.username);
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .map_err(|_| AppError::InternalServerError)?
        .to_string();

    let user_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4) RETURNING id",
        user_id,
        form.username,
        form.email,
        password_hash
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let token = generate_token(record.id);
            Ok(HttpResponse::Ok().json(AuthResponse {
                token,
                user_id: record.id,
            }))
        }
        Err(e) => {
            log::error!("Failed to register user: {:?}", e);
            if e.to_string().contains("unique constraint") {
                return Err(AppError::Conflict("Username or email already exists".to_string()));
            }
            Err(AppError::InternalServerError)
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    pool: web::Data<PgPool>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        form.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| AppError::InternalServerError)?;
            if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
                let token = generate_token(user.id);
                Ok(HttpResponse::Ok().json(AuthResponse {
                    token,
                    user_id: user.id,
                }))
            } else {
                Err(AppError::Unauthorized)
            }
        }
        Ok(None) => Err(AppError::Unauthorized),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            Err(AppError::InternalServerError)
        }
    }
}

pub async fn get_profile(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    use crate::handlers::loans::get_user_id_from_req;
    let user_id = get_user_id_from_req(&req)?;

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_| AppError::NotFound)?;

    #[derive(Serialize)]
    struct ProfileResponse {
        username: String,
        email: String,
        reputation_score: i32,
    }

    Ok(HttpResponse::Ok().json(ProfileResponse {
        username: user.username,
        email: user.email,
        reputation_score: user.reputation_score,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

fn generate_token(user_id: Uuid) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to generate token")
}
