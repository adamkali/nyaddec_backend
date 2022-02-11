use crate::common::DetailedResponse::DetailedResponse;
use crate::models::PartyModel::PartyEntity;
use crate::StdErr;
use crate::repo::PartyRepo::PartyRepo;
use actix_web::web::{Data, Json};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::HttpServiceFactory;
use actix_web::{HttpResponse};

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
    let mut response: DetailedResponse<Vec<PartyEntity>> = DetailedResponse::new();
    
    let data = db.all().await;
    match data {
        Ok(value) => {
            response.data = Some(value);
            response.success = true;
            response.message = format!("APi Request Successful on {}/{}",
                std::env::var("SERVER_URL").unwrap(),
                std::env::var("PARTY_CONTROLLER_PORT").unwrap()
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
}    



