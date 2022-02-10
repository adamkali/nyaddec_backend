use sqlx::{Connection, MySqlConnection, Pool, MySql, mysql::MySqlPoolOptions};
use crate::libs;
use crate::models::PartyModel;

#[derive(Clone)]
pub struct PartyRepo {
    pool: Pool<MySql>
}

impl  PartyRepo {}