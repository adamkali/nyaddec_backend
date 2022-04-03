mod logger;
mod controllers;
mod common;
mod models;
mod repo;
use log::{debug, error, log_enabled, info, Level};

use actix_web::middleware::Logger;
use std::sync::Arc;
use std::sync::Mutex;
use actix_web::web::Data;
use futures::{executor, future};

type StdErr = Box<dyn std::error::Error>;

#[actix_rt::main]
async fn main() -> Result<(), StdErr> {
    // loads env variables from .env
    dotenv::dotenv().ok();


    std::env::set_var("RUST_LOG", "actix_web=trace");
    logger::init()?;

    actix_web::HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            // .allowed_origin(
            //    &(std::env::var("SERVER_URL").unwrap().to_string()+ ":" + &std::env::var("FROTEND").unwrap().to_string())
            // )
            .allow_any_origin()
            .allowed_methods(vec!["GET","POST","PUT"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT
            ])
            .allowed_header(
                actix_web::http::header::CONTENT_TYPE
            )
            .max_age(3600);

        // logger::init();
        
        let party_repo = Data::new(Mutex::new(repo::PartyRepo::PartyRepo::connect()));
        actix_web::App::new()
            .app_data(Data::clone(&party_repo))
            .wrap(Logger::default())
            .wrap(cors)
            .service(controllers::PartyController::party_api())
        })
    .bind((std::env::var("SERVER_URL").unwrap().to_string(),
        std::env::var("PARTY_CONTROLLER_PORT").unwrap()
            .parse::<u16>().unwrap()))?
    .run()
    .await?;

    Ok(())
}
