use rocket::serde::{Deserialize, Serialize};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use rocket_sync_db_pools::rusqlite::{params, Error, Row};

use std::str::FromStr;

use crate::{
    api::errors::ApiError,
    database::{permissions::Permission, Database},
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DangerousLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub permissions: Vec<Permission>,
}

impl User {
    pub fn try_from_row(row: &Row) -> Result<Self, Error> {
        let permissions: Vec<Permission> = row
            .get::<&str, String>("permissions")?
            .split('\u{F1}')
            .filter_map(|s| -> Option<Permission> {
                if s.is_empty() {
                    None
                } else {
                    Permission::from_str(s).ok()
                }
            })
            .collect();
        Ok(User {
            username: row.get("username")?,
            permissions,
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        let db = match request.guard::<Database>().await {
            Outcome::Success(db) => db,
            Outcome::Forward(f) => return Outcome::Forward(f),
            Outcome::Error((e, _)) => {
                return Outcome::Error((Status::InternalServerError, ApiError::Status(e)))
            }
        };

        let token = match request
            .cookies()
            .get("token")
            .map(|val| val.value().to_string())
        {
            Some(v) => v,
            None => return Outcome::Forward(Status::Unauthorized),
        };

        let response = db
            .run(move |conn| {
                conn.query_row(
                    "SELECT users.username, users.permissions FROM tokens
                    LEFT JOIN users ON tokens.username = users.username
                    WHERE tokens.id = ?",
                    params![token],
                    //"SELECT username, COALESCE(GROUP_CONCAT(DISTINCT user_permissions.permission_id)) as permissions FROM tokens
                    //LEFT JOIN users ON tokens.username = user.username
                    //LEFT JOIN user_permissions ON tokens.username = user_permissions.username
                    //WHERE tokens.id = ?",
                    //params![token],
                    User::try_from_row,
                )
            })
            .await;
            dbg!(&response);
        match response {
            Ok(v) => Outcome::Success(v),
            Err(Error::QueryReturnedNoRows) => Outcome::Forward(Status::Unauthorized),
            Err(e) => Outcome::Error((Status::InternalServerError, ApiError::from(e))),
        }
    }
}
