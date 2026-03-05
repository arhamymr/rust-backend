use actix_web::{test, App};
use serde_json::json;

use crate::handlers::auth::refresh;

#[actix_web::test]
async fn test_refresh_success() {
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .to_request();
        
    let app = create_test_app();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), 200); // OK
    
    let body = test::read_body(resp);
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(response_json["message"], "Token refreshed successfully");
    assert!(response_json["token"].is_string());
    assert_eq!(response_json["expires_in"], 3600);
}

// Helper function to create test app with refresh service
fn create_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .service(refresh)
}