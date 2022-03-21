use crate::models::TokenModel;

use std::future::Ready;
use std::pin::Pin;

use actix_web::{FromRequest, HttpRequest};
use actix_web::http::StatusCode;
use actix_web::error::InternalError;
use actix_web::dev::Payload;
use futures::{future, Future, FutureExt};

use crate::StdErr;
use crate::db::Db;
use crate::models::Token;

impl FromRequest for Token {
    type Error = InternalError<&'static str>;
    //type Config = ();

    // we return a Future that is either
    // - immediately ready (on a bad request with a missing or malformed Authorization header)
    // - ready later (pending on a SQL query that validates the request's Bearer token)
    type Future = future::Either<
        future::Ready<Result<Self, Self::Error>>,
        Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + 'static>>,
    >;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // get request headers
        let headers = req.headers();

        // check that Authorization header exists
        let maybe_auth = headers.get("Authorization");
        if maybe_auth.is_none() {
            return future::err(InternalError::new(
                "missing Authorization header",
                StatusCode::BAD_REQUEST,
            ))
            .left_future();
        }

        // check Authorization header is valid utf-8
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

        if (maybe_token_id != std::env::var("BEARER_TOKEN").unwrap().to_string()) {
            return future::err(InternalError::new(
                "invalid bearer token",
                StatusCode::UNAUTHORIZED,
            ))
            .left_future();
        }

        .boxed_local()
        .right_future()
    }
}