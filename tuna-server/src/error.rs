use bcrypt::BcryptError;
use rocket::http::Status;
use rocket_sync_db_pools::rusqlite::{Error as RusqliteError, ErrorCode as RusqliteErrorCode};
use serde_json::Error as JsonError;
use serde_yaml::Error as YamlError;

/// Error returned by the API
#[derive(Debug, Responder)]
pub enum ApiError {
    RusqliteError((Status, String)),
    #[response(status = 500)]
    HashError(String),
    #[response(status = 500)]
    IoError(String),
    #[response(status = 500)]
    SerdeError(String),
    Status(Status),
}

impl From<Status> for ApiError {
    fn from(e: Status) -> Self {
        Self::Status(e)
    }
}

impl From<RusqliteError> for ApiError {
    fn from(e: RusqliteError) -> Self {
        let message = format!("Rusqlite Error: {e}");
        Self::RusqliteError(match e {
            RusqliteError::QueryReturnedNoRows => (Status::NotFound, message),
            RusqliteError::SqliteFailure(error, _)
                if error.code == RusqliteErrorCode::ConstraintViolation =>
            {
                (Status::Conflict, message)
            }
            _ => (Status::InternalServerError, message),
        })
    }
}

impl From<BcryptError> for ApiError {
    fn from(e: BcryptError) -> Self {
        Self::HashError(format!("Hash Error: {e}"))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(format!("IO Error: {e}"))
    }
}

impl From<JsonError> for ApiError {
    fn from(e: JsonError) -> Self {
        Self::SerdeError(format!("Serde JSON Error: {e}"))
    }
}

impl From<YamlError> for ApiError {
    fn from(e: YamlError) -> Self {
        Self::SerdeError(format!("Serde YAML Error: {e}"))
    }
}
