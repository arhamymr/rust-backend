use actix_web::{test, App, web};
use serde_json::json;
use sea_orm::DatabaseConnection;

use crate::handlers::auth::login;

#[actix_web::test]
async fn test_login_success() {
    let login_data = json!({
        "email": "test@example.com",
        "password": "SecureP@ss123"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(login_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 200); // OK
}

#[actix_web::test]
async fn test_login_invalid_email() {
    let login_data = json!({
        "email": "invalid-email",
        "password": "SecureP@ss123"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(login_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400); // Bad Request
}

#[actix_web::test]
async fn test_login_missing_password() {
    let login_data = json!({
        "email": "test@example.com"
    });
    
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(login_data)
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 400); // Bad Request
}

// Helper function to create test app with login service
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
        .service(login)
}