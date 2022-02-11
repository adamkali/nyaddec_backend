use sqlx::MySql;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, sqlx::Decode)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacterEntity {
    pub id: String,
    #[serde(alias = "playerID")]
    pub player_id: i16,
    #[serde(alias = "playerName")]
    pub player_name: String,
    #[serde(alias = "playerHitpoints")]
    pub player_hitpoints: i16,
    #[serde(alias = "playerLevel")]
    pub player_level: i16,
}

impl 
    <'r, DB: sqlx::Database> sqlx::Decode<'r, DB> 
    for PlayerCharacterEntity 
    where &'r str: sqlx::Decode<'r, DB>    
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Vec<PlayerCharacterEntity>, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <Vec<u8> as sqlx::Decode<'_, MySql>>::decode(value)?;

        Ok(value)
    }
} 