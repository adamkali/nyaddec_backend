use crate::models::PlayerCharacterModel::{PlayerCharacterEntity, PlayerCharacterEntityPost};
use sqlx::mysql::MySqlPoolOptions;
use crate::StdErr;
use sqlx::{ Pool, MySql};
//use crate::models::PlayerCharacterModel::PlayerCharacterEntity;

#[derive(Clone)]
pub struct PlayerCharacterRepo {
    pool: Pool<MySql>
}



impl  PlayerCharacterRepo {
    
    pub async fn connect() -> Result<Self, StdErr> {
        let conn = std::env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .connect(&conn)
            .await?;
        Ok( PlayerCharacterRepo { pool })
    }

    pub async fn get_characters_by_fk(&self, fk: String) 
        -> Result<Vec<PlayerCharacterEntity>, StdErr>
    {
        let mut characters = sqlx::query!(
            "SELECT p.id, p.player_id, p.player_name, p.player_hitpoints, p.player_level, p.party_id FROM players p \
            INNER JOIN partys r ON p.party_id = r.id WHERE r.id = ?",
            fk
        )
            .fetch_all(&self.pool)
            .await?;
        let mut res = Vec::<PlayerCharacterEntity>::new();    
        for character in &mut characters {
            let temp = PlayerCharacterEntity {
                id: character.id.clone(),
                player_id: character.player_id.clone(),
                player_name: character.player_name.clone(),
                player_hitpoints: character.player_hitpoints.clone(),
                player_level: character.player_level.clone(),
            };
            res.push(temp);
        }
        Ok(res)
    }

    pub async fn post(self, create_character: PlayerCharacterEntity, fk: String)
        -> Result<PlayerCharacterEntity, StdErr>
    {
        let mut character: PlayerCharacterEntityPost 
            = sqlx::query_as(
                "INSERT INTO players \
                    (id, player_id, player_name, player_hitpoints, player_level, party_id) \
                    VALUES (?,?,?,?,?,?)",
        )
            .bind(create_character.id)
            .bind(create_character.player_id)
            .bind(create_character.player_name)
            .bind(create_character.player_hitpoints)
            .bind(create_character.player_level)
            .bind(fk)
            .fetch_one(&self.pool)
            .await?;
        let res = PlayerCharacterEntity {
            id: character.id.clone(),
            player_id: character.player_id.clone(),
            player_name: character.player_name.clone(),
            player_hitpoints: character.player_hitpoints.clone(),
            player_level: character.player_level.clone(),
        };
        Ok(res)
    }
}