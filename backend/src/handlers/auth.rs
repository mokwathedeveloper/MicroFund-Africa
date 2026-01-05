use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::models::User;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
}

pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(form.password.as_bytes(), &salt)
        .expect("Failed to hash password")
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
            HttpResponse::Ok().json(AuthResponse {
                token,
                user_id: record.id,
            })
        }
        Err(e) => {
            log::error!("Failed to register user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to register user")
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
) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        form.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).expect("Invalid password hash in DB");
            if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_ok() {
                let token = generate_token(user.id);
                HttpResponse::Ok().json(AuthResponse {
                    token,
                    user_id: user.id,
                })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        Err(e) => {
            log::error!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
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