use std::env;
use sea_orm::{DatabaseConnection, Database, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Ok(db)
}


