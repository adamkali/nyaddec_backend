#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacter {
    pub id: char[16],
    #[serde(alias = "playerID")]
    pub player_id: i16,
    #[serde(alias = "playerName")]
    pub player_name: String,
    #[serde(alias = "playerHitpoints")]
    pub player_hitpoints: i16,
    #[serde(alias = "playerLevel")]
    pub player_level: i16,
}