use crate::common::DetailedResponse::DetailedResponse;
use crate::models::PartyModel::PartyEntity;
use crate::StdErr;
use crate::repo::PartyRepo::PartyRepo;
use actix_web::web::{Data, Json, Path};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;
use actix_web::{HttpResponse, Responder};

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

#[actix_web::get("/party")]
async fn parties(db: Data<PartyRepo>) 
    -> Result<
        Json<DetailedResponse<Vec<PartyEntity>>>,
        InternalError<StdErr>
    >
{
    let response: DetailedResponse<Vec<PartyEntity>> = DetailedResponse::new();
    
    let data = db.all().await;
    match data {
        Ok(value) => {
            response.data = data;
            response.success = true;
            response.message = value;
        }
        Err(e) => {
            response.success = false;
            response.message = e;
        }
    }
    data.map(Json)
}

pub fn party_api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/api")
        .service(parties)
}    



