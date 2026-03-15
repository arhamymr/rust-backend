use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait};
use chrono::{NaiveDateTime, Utc};

use crate::utils::errors::CustomError;
use crate::utils::paseto::verify_token;
use crate::entities::user::{self, Entity as User, Column as UserColumn, Model as UserModel, ActiveModel as UserActiveModel};

// GET /api/v1/users/me
// Extract user info from the access token and return user info
// Response example
// {
//      "email": "john@example.com",
//      "name": "John Doe"
// }
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
}

// PUT /api/v1/users/me
// Update user info using the access token and return updated user info
// Request example
// {
//      "name": "John Doe Updated"
// }
// Response example
// {
//      "email": "john@example.com",
//      "name": "John Doe Updated"
// }
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMeRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

fn extract_bearer(req: &HttpRequest) -> Result<String, CustomError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| CustomError::Internal("Authorization header missing".to_string()))?;

    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        Ok(token.to_string())
    } else {
        Err(CustomError::Internal("Invalid Authorization scheme".to_string()))
    }
}

#[get("/api/v1/users/me")]
pub async fn get_me(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder> {
    // 1. Extract Bearer access token
    let token = extract_bearer(&req)?;

    // 2. Verify token and get claims
    let claims = verify_token(&token)?;

    // 3. Extract email from claims
    let email = claims
        .get_claim("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CustomError::Internal("Token missing email claim".to_string()))?
        .to_string();

    // 4. Query user by email
    let user_opt: Option<UserModel> = User::find()
        .filter(UserColumn::Email.eq(&email))
        .one(&*db)
        .await
        .map_err(CustomError::Database)?;

    let user = user_opt.ok_or_else(|| CustomError::Conflict("User not found".to_string()))?;

    let response = UserInfo {
        email: user.email,
        name: user.name,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[put("/api/v1/users/me")]
pub async fn update_me(
    data: web::Json<UpdateMeRequest>,
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder> {
    // 1. Validate input
    data.validate().map_err(CustomError::Validation)?;

    // 2. Extract Bearer access token
    let token = extract_bearer(&req)?;

    // 3. Verify token and get email claim
    let claims = verify_token(&token)?;
    let email = claims
        .get_claim("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CustomError::Internal("Token missing email claim".to_string()))?
        .to_string();

    // 4. Find user by email
    let user_opt: Option<UserModel> = User::find()
        .filter(UserColumn::Email.eq(&email))
        .one(&*db)
        .await
        .map_err(CustomError::Database)?;

    let existing = user_opt.ok_or_else(|| CustomError::Conflict("User not found".to_string()))?;

    // 5. Update name and updated_at
    let now: NaiveDateTime = Utc::now().naive_utc();
    let mut active: UserActiveModel = existing.into();
    active.name = sea_orm::Set(data.name.clone());
    active.updated_at = sea_orm::Set(now);

    let updated = active.update(&*db).await.map_err(CustomError::Database)?;

    let response = UserInfo {
        email: updated.email,
        name: updated.name,
    };

    Ok(HttpResponse::Ok().json(response))
}
