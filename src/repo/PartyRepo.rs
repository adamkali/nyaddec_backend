use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::libs;
use crate::models::PartyModel;

pub struct PartyRepo {
    pool: Pool<MySql>
}

impl PartyRepo {
    pub async fn all(&self) -> Result<Vec<PartyModel>, StdErr> {
        let parties = sqlx::query_as("SELECT * FROM partys")
            .fetch_all(&self.pool)
            .await?;
        Ok(parties)
    }

    pub async fn get(&self, party_id: char[16]) -> Result<PartyModel, StdErr> {
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

    pub async fn delete(&self, party_id: char[16]) -> Result<PartyModel, StdErr> {
        let party = sqlx::query_as("DELETE FROM partys WHERE id = $1")
            .bind(party_id)
            .execute(&self.pool)
            .await?;
        Ok(party);
    }
}