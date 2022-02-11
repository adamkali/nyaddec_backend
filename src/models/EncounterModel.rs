#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub id: String,
    #[serde(alias="encounter_name")]
    pub name: String,
    pub difficulty: String,
    pub monsters: Vec<Monster>,
}