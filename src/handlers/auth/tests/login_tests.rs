use actix_web::{test, App, web};
use serde_json::json;
use sea_orm::DatabaseConnection;

use crate::handlers::auth::register;

#[actix_web::test]
async fn test_register_success() {
    let user_data = json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "SecureP@ss123",
        "name": "Test User"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(user_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 201); // Created
}

#[actix_web::test]
async fn test_register_invalid_email() {
    let user_data = json!({
        "username": "testuser",
        "email": "invalid-email",
        "password": "SecureP@ss123",
        "name": "Test User"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(user_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400); // Bad Request
}

#[actix_web::test]
async fn test_register_weak_password() {
    let user_data = json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "weak",
        "name": "Test User"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(user_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400); // Bad Request
}

#[actix_web::test]
async fn test_register_missing_name() {
    let user_data = json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "SecureP@ss123"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(user_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400); // Bad Request
}

// Helper function to create test app with register service
fn create_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    // Mock database connection for testing
    let db: DatabaseConnection = unsafe { std::mem::zeroed() };
    
    App::new()
        .app_data(web::Data::new(db))
        .service(register)
}