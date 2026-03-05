use actix_web::{
    middleware::{Logger, Compress},
    web
};
use env_logger::Env;

mod handlers;
mod middlewares;
mod db;
mod utils;
mod entities;
mod services;

use crate::handlers::{auth, users};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load env
    dotenvy::dotenv()?;

    use actix_web::{App, HttpServer}; 
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Intialize database connection using sea-orm and run migrations using refinery
    let db = db::connection::connect().await.expect("DB Failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))

            // Authentication and user management handlers
            .service(auth::register)
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout)
            .service(auth::logout_all)
            .service(users::get_me)
            .service(users:: update_me)

            // Another API

    })
    .bind(("127.0.0.1", 4444))?
    .run()
    .await?;

    Ok(())

}