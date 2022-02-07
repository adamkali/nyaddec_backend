#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    pub id: i64,
    #[serde(alias="party_name")]
    pub name: String,
    pub characters: Vec<PlayerCharacter>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCharacter {
    pub id: i64,
    #[serde(alias = "playerID")]
    pub player_id: i16,
    #[serde(alias = "playerName")]
    pub player_name: String,
    #[serde(alias = "playerHitpoints")]
    pub player_hitpoints: i16,
    #[serde(alias = "playerLevel")]
    pub player_level: i16,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub id: i64,
    #[serde(alias="encounter_name")]
    pub name: String,
    pub difficulty: String,
    pub monsters: Vec<Monster>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monster {
    pub id: i64,
    #[serde(alias = "id")]
    pub monster_id: i16,
    #[serde(alias="monster_name")]
    pub name: String,
    pub meta: String,
    pub armor_class: String,
    pub hit_points: String,
    pub speed: String,
    pub strength: String,
    pub str_mod: String,
    pub dexterity: String,
    pub dex_mod: String,
    pub constitution: String,
    pub con_mod: String,
    pub intellegence: String,
    pub int_mod: String,
    pub wisdom: String,
    pub wis_mod: String,
    pub charisma: String,
    pub cha_mod: String,
    pub saving_throw: String,
    pub skills: String,
    pub immune: String,
    pub senses: String,
    pub languages: String,
    pub challenge: String,
    pub traits: String,
    pub actions: String,
    pub legendary: String,
    pub img_url: String,
}