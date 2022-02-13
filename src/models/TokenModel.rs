use std::pin::Pin;

use actix_web::{FromRequest};
use actix_web::{HttpRequest};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::Payload;
use actix_web::web::{Data};

use futures::{future, Future, FutureExt};

use crate::common::Freq::MySqlDB;

// for authentication
#[derive(sqlx::FromRow)]
pub struct Token {
    pub id: String,
    pub expired: chrono::DateTime<chrono::Local>,
}

impl FromRequest for Token {
    type Error = InternalError<&'static str>;
    //type Config = ();

    type Future = future::Either<
        future::Ready<Result<Self, Self::Error>>,
        Pin<Box<dyn Future<
            Output = Result<Self, Self::Error>> + 'static
        >>,
    >;

    fn from_request(
        req: &HttpRequest, _payload: &mut Payload
    ) -> Self::Future 
    {
        let headers = req.headers();

        let maybe_auth = headers.get("Authorization");
        if maybe_auth.is_none() {
            return future::err(InternalError::new(
                "missing Authorization header",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        let auth_config = maybe_auth.unwrap().to_str();
        if auth_config.is_err() {
            return future::err(InternalError::new(
                "malformed Authorization header",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // check Authorization header specifies some authorization strategy
        let mut auth_config_parts = auth_config.unwrap().split_ascii_whitespace();
        let maybe_auth_type = auth_config_parts.next();
        if maybe_auth_type.is_none() {
            return future::err(InternalError::new(
                "missing Authorization type",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // check that authorization strategy is using a bearer token
        let auth_type = maybe_auth_type.unwrap();
        if auth_type != "Bearer" {
            return future::err(InternalError::new(
                "unsupported Authorization type",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // check that bearer token is present
        let maybe_token_id = auth_config_parts.next();
        if maybe_token_id.is_none() {
            return future::err(InternalError::new(
                "missing Bearer token",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // we can fetch managed application data using HttpRequest.app_data::<T>()
        let db = req.app_data::<Data<MySqlDB>>();
        if db.is_none() {
            return future::err(InternalError::new(
                "internal error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
            .left_future();
        }

        let db = db.unwrap().clone();
        let token_id = maybe_token_id.unwrap().to_owned();

        async move {
            db.validate(token_id)
                .await
                .map_err(
                    |_| InternalError::new(
                            "invalid Bearer tokrn",
                            StatusCode::UNAUTHORIZED
                        )
                    )
        }
        .boxed_local()
        .right_future()
    }
}
