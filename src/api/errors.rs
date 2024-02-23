use bcrypt::BcryptError;
use rocket::http::Status;
use rocket_sync_db_pools::rusqlite::Error as RusqliteError;

#[derive(Debug, Responder)]
pub enum ApiError {
    RusqliteError((Status, String)),
    #[response(status = 500)]
    HashError(String),
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
            _ => (Status::InternalServerError, message),
        })
    }
}

impl From<BcryptError> for ApiError {
    fn from(e: BcryptError) -> Self {
        Self::HashError(format!("Hash Error: {e}"))
    }
}
