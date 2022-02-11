#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, sqlx::Type)]
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