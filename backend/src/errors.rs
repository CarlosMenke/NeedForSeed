use actix_web::{error::ResponseError, Error as ActixWebError, HttpResponse};
use argon2::password_hash::Error as ArgonError;
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use serde_json::Error as SerdeJsonError;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    //TODO map to actix unotharized
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError("DBError from diesel".to_string())
            }
            _ => ServiceError::InternalServerError("DBError from diesel".to_string()),
        }
    }
}

impl From<ArgonError> for ServiceError {
    fn from(error: ArgonError) -> ServiceError {
        match error {
            _ => ServiceError::InternalServerError("ArgonError from Argon".to_string()),
        }
    }
}

impl From<SerdeJsonError> for ServiceError {
    fn from(error: SerdeJsonError) -> ServiceError {
        match error {
            _ => {
                ServiceError::InternalServerError("Error for converting serde to json".to_string())
            }
        }
    }
}

impl From<ActixWebError> for ServiceError {
    fn from(error: ActixWebError) -> ServiceError {
        match error {
            _ => ServiceError::InternalServerError("Failed to create login Token".to_string()),
        }
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> ServiceError {
        match error {
            _ => ServiceError::InternalServerError("std::io Error [File interaction]".to_string()),
        }
    }
}
