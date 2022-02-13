use sqlx::query_as;
use super::PlayerCharacterRepo;
use sqlx::{ Pool, MySql, mysql::MySqlPoolOptions};
use crate::models::{
    PartyModel::{PartyEntity, PartyEntityPost},
    PlayerCharacterModel::PlayerCharacterEntity
};
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

    pub async fn link(&self, i: String) -> 
        Result<Vec<PlayerCharacterEntity>, StdErr> 
    {
        let pcr = PlayerCharacterRepo::PlayerCharacterRepo::connect().await?;
        let res = pcr.get_characters_by_fk(i).await?;
        Ok(res)
    }

    pub async fn also(&self, character: &PlayerCharacterEntity, fk: String) 
        -> Result<PlayerCharacterEntity,StdErr>
    {
        let pcr = PlayerCharacterRepo::PlayerCharacterRepo::connect().await?;
        let res = pcr.post(character.clone(), fk).await?;
        Ok(res)
    }

    pub async fn all(&self) -> Result<Vec<PartyEntity>, StdErr> {
        let mut parties = sqlx::query!(
            "SELECT * FROM partys"
        )
            .fetch_all(&self.pool)
            .await?;

        let mut res = Vec::<PartyEntity>::new();
        
        for party in &mut parties {
            let characters = self.link(party.id.clone()).await?;
            let temp = PartyEntity {
                id: party.id.clone(),
                party_name: party.party_name.clone(),
                characters: characters,
            };
            res.push(temp);
        }
        Ok(res)
    }

    pub async fn get(&self, party_id: String) -> Result<PartyEntity, StdErr> {
        let party = sqlx::query!(
            "SELECT * FROM partys WHERE id = ?",
            party_id
        )
            .fetch_one(&self.pool)
            .await?;
        let characters = self.link(party.id.clone()).await?;
        let res = PartyEntity {
            id: party.id.clone(),
            party_name: party.party_name.clone(),
            characters: characters,
        };
        Ok(res)
    }

    pub async fn post(&self, mut create_party: PartyEntity) -> Result<PartyEntity, StdErr> {
        let party: PartyEntityPost = query_as(
            "INSERT INTO partys (id, party_name) VALUES (?, ?)",
        )
            .bind(create_party.id)
            .bind(create_party.party_name)
            .fetch_one(&self.pool)
            .await?;
        let players = Vec::<PlayerCharacterEntity>::new();
        let mut res: PartyEntity = PartyEntity {
            id: party.id.clone(),
            party_name: party.party_name.clone(),
            characters: players,
        };
        for character in &mut create_party.characters {
            res.characters.push(self.also(character, party.id.clone()).await?);
        }
        Ok(res) 
    }

    pub async fn delete(&self, party_id: String) 
        -> Result<(), StdErr> 
    {
        sqlx::query!("DELETE FROM partys WHERE id = ?", party_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}