use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("jwt token not valid")]
    InvalidJWTToken,
    #[error("jwt token creation error ")]
    JWTTokenCreation,
    #[error("no auth error")]
    NoAuthHeader,
    #[error("invalid auth header")]
    InvalidAuthHeader,
    #[error("no permission")]
    NoPermission,
}

impl warp::reject::Reject for Error {}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::WrongCredentials => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermission => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::InvalidJWTToken => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}
