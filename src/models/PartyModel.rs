use super::PlayerCharacterModel::PlayerCharacterEntity;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PartyEntity {
    pub id: String,
    #[serde(alias="party_name")]
    pub name: String,
    pub characters: Vec<PlayerCharacterEntity>,
}
