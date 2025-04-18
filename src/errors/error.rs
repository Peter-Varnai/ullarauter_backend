use actix_web::{error::Error as ActixError, HttpResponse, ResponseError};
use askama::Error as AskamaError;
use std::{fmt, io::Error as StdError};
use actix_session::SessionInsertError;
use serde_json::Error as JsonError;
use sqlx::Error as SqlxError;


#[derive(Debug)]
pub enum HandlerError {
    Actix(ActixError),
    Askama(AskamaError),
    Session(SessionInsertError),
    Json(JsonError),
    Sqlx(SqlxError),
    Std(StdError),
}


impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandlerError::Actix(err) => write!(f, "Actix error: {}", err),
            HandlerError::Askama(err) => write!(f, "Askama error: {}", err),
            HandlerError::Session(err) => write!(f, "Session error: {}", err),
            HandlerError::Json(err) => write!(f, "Json string conversion error: {}", err),
            HandlerError::Std(err) => write!(f, "Standard library error: {}", err),
            HandlerError::Sqlx(err) => write!(f, "Sqlx error: {}", err),
        }
    }
}

impl ResponseError for HandlerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            HandlerError::Actix(err) => 
                err.error_response(),
            HandlerError::Std(err) =>
                HttpResponse::InternalServerError().body(format!("Standard library error: {}", err)),
            HandlerError::Askama(err) => 
                HttpResponse::InternalServerError().body(format!("Template error: {}", err)),
            HandlerError::Session(err) => 
                HttpResponse::InternalServerError().body(format!("Insert Session error: {}", err)),
            HandlerError::Json(err) =>
                HttpResponse::InternalServerError().body(format!("Json conversion error: {}", err)),
             HandlerError::Sqlx(err) =>
                HttpResponse::InternalServerError().body(format!("Sqlx query/insert error: {}", err)),
        }
    }
}


impl std::error::Error for HandlerError {}

impl From<StdError> for HandlerError {
    fn from(err: StdError) -> Self {
        HandlerError::Std(err)
    }
}

impl From<AskamaError> for HandlerError {
    fn from(err: AskamaError) -> Self {
        HandlerError::Askama(err)
    }
}

impl From<SessionInsertError> for HandlerError {
    fn from(err: SessionInsertError) -> Self {
        HandlerError::Session(err)
    }
}

impl From<ActixError> for HandlerError {
    fn from(err: ActixError) -> Self {
        HandlerError::Actix(err)
    }
}

impl From<JsonError> for HandlerError {
    fn from(err: JsonError) -> Self {
        HandlerError::Json(err)
    }
}

impl From<SqlxError> for HandlerError {
    fn from(err: SqlxError) -> Self {
        HandlerError::Sqlx(err)
    }
}




