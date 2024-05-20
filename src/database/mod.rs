use dotenvy;
use std::env;

pub fn connect_pgsql() {
    // connect to postgresql using diesel
    dotenvy::dotenv().expect("Failed to load .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("database_url: {}", database_url);
    println!("connect to postgresql");
}
