//use std::sync::Arc;
use actix_web::http::header::ContentType;
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
use log::{debug};


use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::web::{Data, Json, Path};
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;

#[actix_web::get("/parties")]
async fn parties(
    request: HttpRequest,
    _t: Token
) 
    -> HttpResponse
{
    let mut response: DetailedResponse<Vec<PartyEntity>> = DetailedResponse::new();
    
    debug!("Fired parties");

    let db = request.app_data::<Data::<Mutex::<PartyRepo>>>().unwrap();
    let repo = db.lock().unwrap().clone();
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
    debug!("Response from server: {:?}", Json(&response));
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .insert_header(("X-Hdr", "sample"))
        .body(response);
}

#[actix_web::get("/party/{party_id}")]
async fn party_get(
    request: HttpRequest,
    party_id : Path<String>,
    _t: Token
) 
    -> Result<impl Responder, InternalError<StdErr>>
{
    let mut response: DetailedResponse<PartyEntity> 
        = DetailedResponse::new();
    
    let db = request.app_data::<Data::<Mutex::<PartyRepo>>>().unwrap();
    let repo = db.lock().unwrap().clone();
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
        }
        Err(e) => {
            response.success = false;
            response.message = e.to_string();
        }
    }
    return Ok(Json(response));
}

#[actix_web::post("/party/post")]
async fn party_post(
    request: HttpRequest,
    create_party: Json<PartyEntity>,
    _t: Token
)
    -> Result<impl Responder, InternalError<StdErr>>
{

    
    let post_party = PartyEntity {
        id: generate_guid(),
        party_name: create_party.0.party_name,
        characters: create_party.0.characters,
    };
    let db = request.app_data::<Data::<Mutex::<PartyRepo>>>().unwrap();
    let repo = db.lock().unwrap().clone();
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



