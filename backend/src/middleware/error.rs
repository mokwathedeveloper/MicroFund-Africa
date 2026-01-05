use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Not Found")]
    NotFound,

    #[display(fmt = "Conflict: {}", _0)]
    Conflict(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal Server Error".to_string(),
                })
            }
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: message.clone(),
                })
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Unauthorized".to_string(),
                })
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "Resource Not Found".to_string(),
                })
            }
            AppError::Conflict(ref message) => {
                HttpResponse::Conflict().json(ErrorResponse {
                    error: message.clone(),
                })
            }
        }
    }
}
