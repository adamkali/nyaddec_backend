use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::common::Freq;
use crate::models::PartyModel;
use crate::StdErr;

#[derive(Clone)]
pub struct PartyRepo {
    pool: Pool<MySql>
}

impl PartyRepo {

    pub async fn connect() -> Result<Self, StdErr> {
        let conn = std::env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .connect(&conn)
            .await?;
        Ok( PartyRepo { pool } )
    }

    pub async fn all(&self) -> Result<Vec<PartyModel>, StdErr> {
        let parties = sqlx::query_as("SELECT * FROM partys")
            .fetch_all(&self.pool)
            .await?;
        Ok(parties)
    }

    pub async fn get(&self, party_id: &str) -> Result<PartyModel, StdErr> {
        let party = sqlx::query_as("SELECT * FROM partys WHERE id = $1")
            .bind(party_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(party)
    }

    pub async fn post(&self, create_party: PartyModel) -> Result<PartyModel, StdErr> {
        let party = sqlx::query_as("INSERT INTO partys (id, party_name)\
                                    VALUES ($1, $2)")
            .bind(create_party.id)
            .bind(create_party.name)
            .fetch_one(&self.pool)
            .await?;
        Ok(party) 
    }

    pub async fn delete(&self, party_id: &str) 
        -> Result<(), StdErr> 
    {
        sqlx::query("DELETE FROM partys WHERE id = $1")
            .bind(party_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}