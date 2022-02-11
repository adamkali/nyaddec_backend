use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::models::*;
use crate::StdErr;

pub struct Db {
    pool: Pool<MySql>,
}

impl Db {
    pub async fn connect() -> Result<self, StdErr> {
        let db_url = std::env::var("DATABASE_URL");
        let pool = MySqlPoolOptions::new().connect(&db_url).await?;
        Ok(Db { pool });
    }

    pub async fn partys(&self) -> Result<Vec<Party>, StdErr> {
        let partys = sqlx::query_as("SELECT * FROM partys")
            .fetch_all(&self.pool)
            .await?;
        Ok(partys)
    }
}