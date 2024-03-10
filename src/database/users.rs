use rocket::serde::{Deserialize, Serialize};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows};

use crate::{
    api::errors::ApiError,
    database::{permissions::Permission, Database},
};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DangerousLogin {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct DangerousUser {
    pub username: String,
    pub permissions: SqlVec<Permission>,
    #[serde(skip_serializing)]
    pub hash: String,
    #[serde(skip_serializing)]
    pub sessions: SqlVec<String>,
}

impl DangerousUser {
    pub fn has_permissions(&self, permissions: &[Permission]) -> bool {
        let self_permissions = self.permissions.inner();
        for permission in permissions {
            if !self_permissions.contains(permission) {
                return false;
            }
        }
        true
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DangerousUser {
    type Error = ApiError;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<DangerousUser, Self::Error> {
        let db = match request.guard::<Database>().await {
            Outcome::Success(db) => db,
            Outcome::Forward(f) => return Outcome::Forward(f),
            Outcome::Error((e, _)) => {
                return Outcome::Error((Status::InternalServerError, ApiError::Status(e)))
            }
        };

        let cookie_val = match request
            .cookies()
            .get("session")
            .map(|val| val.value().to_string())
        {
            Some(v) => v,
            None => return Outcome::Forward(Status::Unauthorized),
        };

        match db
            .run(move |conn| {
                conn.query_row(
                    "SELECT * FROM users WHERE sessions LIKE ?",
                    params![format!("%{}%", cookie_val)],
                    DangerousUser::try_from_row,
                )
            })
            .await
        {
            Ok(v) => Outcome::Success(v),
            Err(QueryReturnedNoRows) => Outcome::Forward(Status::Unauthorized),
            Err(e) => Outcome::Error((Status::InternalServerError, ApiError::from(e))),
        }
    }
}
