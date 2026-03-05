use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("validation error")]
    Validation(ValidationErrors),

    #[error("database error")]
    Database(DbErr),

    #[error("internal server error")]
    Internal(String),

    #[error("conflict error")]
    Conflict(String),

    #[error("Token error: {0}")]
    Token(#[from] pasetors::errors::Error),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::Validation(e) => HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "message": "Validation failed",
                "errors": e,
            })),

            CustomError::Database(e) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": "Database error",
                    "details": e.to_string()
                }))
            }

            CustomError::Internal(msg) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": msg,
                }))
            }

            CustomError::Conflict(msg) => HttpResponse::Conflict().json(serde_json::json!({
                "status": "error",
                "message": msg,
            })),

            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
