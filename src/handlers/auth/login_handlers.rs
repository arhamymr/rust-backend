use actix_web::{web, post, HttpResponse, Responder, Result };
use sea_orm::DatabaseConnection;
use validator::Validate;

use crate::utils::errors::CustomError;
use crate::services::user_service::{do_login, LoginUserData};
use super::models::{LoginRequest, LoginResponse};

#[post("/api/v1/auth/login")]
pub async fn login(
    data: web::Json<LoginRequest>,
    db: web::Data<DatabaseConnection>
) -> Result<impl Responder> {

    // Validate input
    data.validate().map_err(CustomError::Validation)?;

    let login_data = LoginUserData {
        email: data.email.clone(),
        password: data.password.clone()
    };

    match do_login(login_data, &db).await {
        Ok(token) => {
            let response = LoginResponse {
                message: "User logged in successfully".to_string(),
                expires_in: 3600,
                token
            };

            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            match err {
                CustomError::Internal(msg) if msg.contains("already exists") => {
                    Ok(HttpResponse::Conflict().json(serde_json::json!({
                        "status": "error",
                    })))
                }
                _ => Err(err.into())
            }
        }
    }
}
