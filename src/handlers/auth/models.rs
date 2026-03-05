use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

// POST /api/v1/auth/register
// request example
// {
//      "username": "john_doe",
//      "email": "john@example.com"
//      "password": "securepassword123"
//      "name": "John Doe"
// }
// response example
// {
//      "message": "User registered successfully",
// }

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(
        length(min = 8, message = "Password must be at least 8 characters"),
        custom(function = "validate_password")
    )]
    pub password: String,

    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

// POST /api/v1/auth/login
// request example
// {
//      "email": "john@example.com"
//      "password": "securepassword123"
// }
// response example
// {
//      "message": "User logged in successfully",
//      "token": "eyJhbGciOi(...) paseto token ",
//      "expires_in": 3600
// }

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub expires_in: u64,
}

// POST /api/v1/auth/refresh
// refresh token get from cookie and return new access token
// response example
// {
//      "message": "Token refreshed successfully",
//      "token": "eyJhbGciOi(...) paseto token ",
//      "expires_in": 3600
// }

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub message: String,
    pub token: String,
    pub expires_in: u64,
}

// POST /api/v1/auth/logout
// Flow
// Get refresh token from cookie and invalidate it
// Mark as revoked in db
// Clear cookie
// response example
// {
//      "message": "User logged out successfully"
// }

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutAllResponse {
    pub message: String,
}

// Custom password validation
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric()); // Fixed: check for non-alphanumeric (special characters)

    if has_uppercase && has_lowercase && has_digit && has_special {
        Ok(())
    } else {
        let mut err = ValidationError::new("weak_password");
        err.message = Some(
            "Password must contain uppercase, lowercase, number, and special character".into(),
        );
        Err(err)
    }
}
