mod user;

use sqlx::{Pool, Postgres, postgres::{PgPoolOptions, PgDone}};
use anyhow::Result;

pub struct DatabaseContext {
    pool: Pool<Postgres>
}

impl DatabaseContext {
    pub async fn new() -> Result<DatabaseContext> {
        let credentials = PostgreSQLCredentials::new()?;

        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(&credentials.to_connection_string())
            .await?;

        let database_context = DatabaseContext{ pool };
        database_context.init_tables().await?;

        database_context.create_user(crate::models::user::User::new("Jimmy", "123abc456", "jimmy@gmail.com")).await?;
        database_context.create_user(crate::models::user::User::new("Johnny", "abc123def", "johnny@gmail.com")).await?;

        Ok(database_context)
    }

    async fn init_tables(&self) -> Result<PgDone> {
        self.init_user_table().await?;

        Ok(PgDone::default())
    }

}
struct PostgreSQLCredentials {
    host: String,
    port: String,
    db: String,
    user: String,
    password: String,
}

impl PostgreSQLCredentials {
    fn new() -> Result<Self> {
        let credentials = PostgreSQLCredentials {
            host: std::env::var("POSTGRES_HOST")?,
            port: std::env::var("POSTGRES_PORT")?,
            db: std::env::var("POSTGRES_DB")?,
            user: std::env::var("POSTGRES_USER")?,
            password: std::env::var("POSTGRES_PASSWORD")?,
        };

        Ok(credentials)
    }
    
    fn to_connection_string(&self) -> String {
        format!("postgresql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.db)
    }
}
