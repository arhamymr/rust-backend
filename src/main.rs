use actix_web::{
    get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

mod auth;
mod database;

#[derive(Deserialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

async fn token(req_body: web::Json<TokenRequest>) -> impl Responder {
    // body name, client_id, client_secret, and grant_type

    let client_id = &req_body.client_id;
    let client_secret = &req_body.client_secret;
    let grant_type = &req_body.grant_type;

    println!("client_id: {}", client_id);

    // TODO !!
    // check client_id and client_secret in database
    if client_id != "client_id" || client_secret != "client_secret" {
        return HttpResponse::Unauthorized().body("invalid client_id or client_secret");
    }

    // generate token based on grant_type
    let token = match grant_type.as_str() {
        "client_credentials" => "client_token",
        "password" => "password_token",
        "refresh_token" => "refresh_token",
        _ => "invalid_grant_type",
    };

    HttpResponse::Ok().body(token)
}

#[derive(Deserialize)]
struct ClientData {
    name: String,
}

#[derive(Serialize)]
struct ResponseData {
    client_id: String,
    client_secret: String,
}

#[post("/generate-credentials")]
async fn generate_credentials(data: Json<ClientData>) -> impl Responder {
    let (client_id, client_secret) = auth::generate_credentials(&data.name.to_string());
    Json(ResponseData {
        client_id,
        client_secret,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::connect_pgsql();

    HttpServer::new(|| App::new().service(generate_credentials))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    println!("Server is running at http://127.0.0.1:8080");

    Ok(())
}
