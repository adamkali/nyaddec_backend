use super::PlayerCharacterModel::PlayerCharacter;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyEntity {
    pub id: str,
    #[serde(alias="party_name")]
    pub name: String,
    pub characters: Vec<PlayerCharacter>,
}
