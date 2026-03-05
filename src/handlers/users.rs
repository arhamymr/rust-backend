use actix_web::{get, put, HttpResponse, Responder};
use serde::Serialize;

// GET /api/v1/users/me 
// Get user info from access token and return user info
// response example
// {
//      "username": "john_doe",
//      "email": "emailexample@mail.com",
//      "name": "John Doe"
// }

#[get("/api/v1/users/me")]
pub async fn get_me() -> impl Responder {
    HttpResponse::Ok().body("User info")
}

// PUT /api/v1/users/me
// Update user info from access token and return updated user info
// request example
// {
//      "name": "John Doe Updated"
// }
// response example
// {
//      "username": "john_doe",
//      "email": "
//      "name": "John Doe Updated"
// }


#[derive(Serialize)]
struct UserInfo {
    username: String,
    email: String,
    name: String,
}

#[put("/api/v1/users/me")]
pub async fn update_me() -> impl Responder {

    let updated_user = UserInfo {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        name: "John Doe Updated".to_string(),
    };

    HttpResponse::Ok().json(updated_user)
}
