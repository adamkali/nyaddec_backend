use sqlx::{Connection, MySqlConnection};
use crate::models::PartyModel;
use crate::StdErr;


fn to_internal_error(e: StdErr) -> Result<InternalError<StdErr>> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR);
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}
