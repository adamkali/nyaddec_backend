use rand::Rng;
use sqlx::{ Pool, MySql, mysql::MySqlPoolOptions};
use crate::StdErr;

pub fn generate_guid() -> String {
    const CHARSET: &[u8] = b"0123456789abcdef";
    const GUIDLEN: usize = 16; 
    
    let mut rng = rand::thread_rng();
    let guid: String = (0..GUIDLEN)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    return guid;
}

pub struct MySqlDB {
    pool: Pool<MySql>,
}

impl MySqlDB {
    pub async fn connect() -> Result<Self, StdErr> {
        let conn = std::env::var("DATABASE_URL")?;
        let pool = MySqlPoolOptions::new()
            .connect(&conn)
            .await?;
        Ok(MySqlDB { pool} )
    }
}