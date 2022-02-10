use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::common::DetailedResponse::DetailedResponse;
use crate::models::models::PartyEntity;

#[derive(Clone)]
pub struct PlayerCharacterRepo {
    pool: Pool<MySql>
}

impl  PlayerCharacterRepo {}