use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use std::sync::Mutex;
use crate::common::{
    DetailedResponse::DetailedResponse,
    Freq::generate_guid
};
use crate::models::{ 
    PartyModel::PartyEntity,
    TokenModel::Token
};
use crate::StdErr;
use crate::repo::PartyRepo::PartyRepo;

use actix_web::web::{Data, Json, Path};
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;

#[actix_web::get("/parties")]
async fn parties(
    db: Data<Mutex<PartyRepo>>,
    _t: Token
) 
    -> Result<
        Json<DetailedResponse<Vec<PartyEntity>>>,
        InternalError<StdErr>,
    >
{
    let mut response: DetailedResponse<Vec<PartyEntity>> = DetailedResponse::new();
    
    let repo = db.lock().unwrap();
    let data = repo.all().await;
    match data {
        Ok(value) => {
            response.data = Some(value);
            response.success = true;
            response.message = format!("APi Request Successful on {}/{}/{}",
                std::env::var("SERVER_URL").unwrap(),
                std::env::var("PARTY_CONTROLLER_PORT").unwrap(),
                "parties"
            );
        }
        Err(e) => {
            response.success = false;
            response.message = e.to_string();
        }
    }
    Ok(Json(response))
}

#[actix_web::get("/party/{party_id}")]
async fn party_get(
    db: Data<Mutex<PartyRepo>>,
    party_id : Path<String>,
    _t: Token
) 
    -> impl Responder
{
    let mut response: DetailedResponse<PartyEntity> 
        = DetailedResponse::new();
    
    let repo = db.lock().unwrap();
    let data = repo.get(party_id.to_string()).await;
    match data {
        Ok(value) => {
            response.data = Some(value);
            response.success = true;
            response.message = format!("APi Request Successful on {}/{}/{}",
                std::env::var("SERVER_URL").unwrap(),
                std::env::var("PARTY_CONTROLLER_PORT").unwrap(),
                "party/{party_id}",
            );
        
        let ok = StatusCode::from_u16(200).unwrap();
        HttpResponse::with_body(ok,Json(response));
        }
        Err(e) => {
            response.success = false;
            response.message = e.to_string();
        }
    }
    HttpResponse::Ok();
}

#[actix_web::post("/party/post")]
async fn party_post(
    db: Data<Mutex<PartyRepo>>,
    create_party: Json<PartyEntity>,
    _t: Token
)
    -> Result<
        Json<DetailedResponse<PartyEntity>>,
        InternalError<StdErr>
    >
{

    let post_party = PartyEntity {
        id: generate_guid(),
        party_name: create_party.0.party_name,
        characters: create_party.0.characters,
    };
    let repo = db.lock().unwrap();
    let data = repo.post(post_party).await;

    let mut response: DetailedResponse<PartyEntity>
        = DetailedResponse::new();


    match data {
        Ok(value) => {
            response.data = Some(value);
            response.success = true;
            response.message = format!("APi Request Successful on {}/{}/{}",
                std::env::var("SERVER_URL").unwrap(),
                std::env::var("PARTY_CONTROLLER_PORT").unwrap(),
                "party/post",
            );
        }
        Err(e) => {
            response.success = false;
            response.message = e.to_string();
        }
    }

    Ok(Json(response))
}

pub fn party_api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/api")
        .service(parties)
        .service(party_get)
        .service(party_post)
}    



