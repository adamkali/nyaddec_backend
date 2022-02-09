#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: char[16],
    #[serde(alias="party_name")]
    pub name: String,
    pub characters: Vec<PlayerCharacter>,
}
