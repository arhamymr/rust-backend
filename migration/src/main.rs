use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env if present (searches current dir and parents)
    let _ = dotenvy::dotenv();
    cli::run_cli(migration::Migrator).await;
}
