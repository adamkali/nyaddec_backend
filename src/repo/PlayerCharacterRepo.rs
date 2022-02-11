use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::common::DetailedResponse::DetailedResponse;
use crate::models::PlayerCharacterModel::PlayerCharacterEntity;
use crate::StdErr;

#[derive(Clone, sqlx::Decode)]
pub struct PlayerCharacterRepo {
    pool: Pool<MySql>
}



impl  PlayerCharacterRepo {}