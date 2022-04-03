use crate::common::StructSize::StructSize;
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

impl std::fmt::Debug for PartyEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("ID", &self.id)
            .field("Party Name", &self.party_name)
            .field("Player Characters", &self.characters)
            .finish()
    }
}

impl StructSize for PartyEntity {
    fn get_size(&self) -> u64 {
        for i in 0..self.characters.len() {
            
        }
        return self.id.len() as u64 + self.party_name.len() as u64
    }
}
