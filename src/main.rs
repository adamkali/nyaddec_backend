mod logger;
mod controllers;
mod common;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv()?;
    logger::init()?;

    let db = common::Freq::MySqlDB::connect().await?;  

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(db.clone())
            .service(controllers::PartyController::party_api())
        })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
