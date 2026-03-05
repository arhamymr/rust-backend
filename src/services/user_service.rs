use actix_web::{Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

use crate::entities::{user, account};
use crate::utils::errors::CustomError;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::paseto::create_token;

pub struct CreateUserData {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

pub async fn create_user_with_account(
    data: CreateUserData,
    db: &DatabaseConnection,
) -> Result<(), CustomError> {
    let transaction = db.begin().await.map_err(CustomError::Database)?;
    
    // Check if user with this email already exists
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(&data.email))
        .one(&transaction)
        .await
        .map_err(CustomError::Database)?;
    
    if existing_user.is_some() {
        return Err(CustomError::Conflict("User with this email already exists".to_string()));
    }
    
    // Create user model
    let now: NaiveDateTime = Utc::now().naive_utc();
    let user_model = user::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        name: Set(data.name),
        email: Set(data.email),
        email_verified: Set(false),
        image: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        role: Set(Some("user".to_string())),
        banned: Set(None),
        ban_reason: Set(None),
        ban_expires: Set(None),
    };
    
    let inserted_user = user_model.insert(&transaction)
        .await
        .map_err(CustomError::Database)?;
    
    // Hash password
    let hashed_password = hash_password(&data.password)?;
    
    // Create account for email/password authentication
    let now: NaiveDateTime = Utc::now().naive_utc();
    let account_model = account::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        account_id: Set(inserted_user.email.clone()),
        provider_id: Set("email".to_string()),
        user_id: Set(inserted_user.id.clone()),
        access_token: Set(None),
        refresh_token: Set(None),
        id_token: Set(None),
        access_token_expires_at: Set(None),
        refresh_token_expires_at: Set(None),
        scope: Set(None),
        password: Set(Some(hashed_password)),
        created_at: Set(now),
        updated_at: Set(now),
    };
    
    account_model.insert(&transaction)
        .await
        .map_err(CustomError::Database)?;
    
    transaction.commit().await.map_err(CustomError::Database)?;
    
    Ok(())
}


pub struct LoginUserData {
    pub email: String,
    pub password: String
}

pub async fn do_login(
    data: LoginUserData,
    db: &DatabaseConnection,
) -> Result<String, CustomError> {
    let transaction = db.begin().await.map_err(CustomError::Database)?;

    let finded_user = user::Entity::find()
    .filter(user::Column::Email.eq(&data.email))
    .one(&transaction)
    .await
    .map_err(CustomError::Database)?;

    let user = finded_user.ok_or_else(||{
        CustomError::Conflict("User not found".to_string())
    })?;

    let finded_account = account::Entity::find()
    .filter(account::Column::UserId.eq(&user.id))
    .one(&transaction)
    .await.map_err(CustomError::Database)?;


    let password_hash = finded_account
    .and_then(|account| account.password)
    .ok_or_else(||{
        CustomError::Internal("Password not set".to_string())
    })?;

    let is_valid = verify_password(&data.password, &password_hash)?;

    if !is_valid {
        return Err(CustomError::Internal("Invalid credentials".to_string()));
    }
    
    // 3. Generate JWT/PASETO token
    let token = match create_token(&user) {
        Ok(token) => token,
        Err(e) => {
            println!("Create token error: {}", e);
            return Err(e);
        }
    };

    transaction.commit().await.map_err(CustomError::Database)?;
    Ok(token)
}
