use actix_web::HttpResponse;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Connection to database failed: {0}")]
    DatabaseConnFailed(String),
    #[error("Database Txn failed: {0}")]
    DbTxnFailure(String),
    #[error("Request failed")]
    RequestFailed,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::DatabaseConnFailed(ref message) => HttpResponse::InternalServerError()
                .body(format!("Database connection failed: {}", message)),
            ApiError::DbTxnFailure(ref message) => HttpResponse::InternalServerError()
                .body(format!("Database transaction failed: {}", message)),
            ApiError::RequestFailed => HttpResponse::BadRequest().body("Request failed"),
        }
    }
}
