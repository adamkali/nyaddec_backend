use super::PlayerCharacterModel::PlayerCharacterEntity;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, sqlx::Decode)]
#[serde(rename_all = "camelCase")]
pub struct PartyEntity {
    pub id: String,
    #[serde(alias = "name")]
    pub party_name: String,
    pub characters: Vec<PlayerCharacterEntity>,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow, sqlx::Decode, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub struct PartyEntityPost {
    pub id: String,
    pub party_name: String,
}
