use sqlx::{ Decode};


#[derive(serde::Serialize, serde::Deserialize, Decode, sqlx::Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacterEntity {
    pub id: String,
    #[serde(alias = "playerID")]
    pub player_id: i64,
    #[serde(alias = "playerName")]
    pub player_name: String,
    #[serde(alias = "playerHitpoints")]
    pub player_hitpoints: i64,
    #[serde(alias = "playerLevel")]
    pub player_level: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Decode, sqlx::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacterEntityPost {
    pub id: String,
    #[serde(alias = "playerID")]
    pub player_id: i64,
    #[serde(alias = "playerName")]
    pub player_name: String,
    #[serde(alias = "playerHitpoints")]
    pub player_hitpoints: i64,
    #[serde(alias = "playerLevel")]
    pub player_level: i32,
    pub fk: String,
}

/*
impl <'r, Db: sqlx::Database> Decode<'_, Db> 
    for Vec<PlayerCharacterEntity>
    where &'r str: Decode<'r, Db> {
    
    fn decode(
        value: <Db as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Vec<PlayerCharacterEntity>, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Db>>::decode(value)?;

        Ok(value.parse()?)
    }
}
*/    