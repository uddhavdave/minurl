use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("connection to database failed")]
    DatabaseConnFailed,
    #[error("Request failed")]
    RequestFailed,
}
