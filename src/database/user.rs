use anyhow::Result;
use sqlx::query_as;

use crate::models::user::*;

impl super::DatabaseContext {
    pub(super) async fn init_user_table(&self) -> Result<sqlx::postgres::PgDone> {
        let query = "\
        CREATE TABLE IF NOT EXISTS customer ( \
            id SERIAL PRIMARY KEY, \
            name TEXT, \
            password TEXT, \
            email TEXT \
        )";

        Ok(sqlx::query(query).execute(&self.pool).await?)
    }

    pub(super) async fn create_user(&self, new_user: User) -> Result<User>{
        let query = "\
        INSERT INTO customer (name, password, email) \
            VALUES ($1, $2, $3) \
            RETURNING *";

        let row = query_as::<_, User>(query)
            .bind(new_user.name)
            .bind(new_user.password)
            .bind(new_user.email)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(User { id: row.id, name: row.name, password: row.password, email: row.email })
    }

    pub async fn get_user(&self, id: i32) -> Result<Option<User>>{
        let query = "SELECT * FROM customer WHERE id = $1";

        let row = query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        
        Ok(row)
    }
}