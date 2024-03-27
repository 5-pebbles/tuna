use bcrypt::{hash, DEFAULT_COST};
use rocket::serde::{Deserialize, Serialize};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};
use rocket_sync_db_pools::rusqlite::{params, params_from_iter, Error, Row, Transaction};

use crate::{
    api::data::permissions::{permissions_from_row, Permission},
    error::ApiError,
    database::MyDatabase,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DangerousLogin {
    pub username: String,
    pub password: String,
}

impl DangerousLogin {
    pub fn insert_user_into_transaction<T: IntoIterator<Item = Permission>>(
        &self,
        permissions_iter: T,
        tx: &Transaction<'_>,
    ) -> Result<(), ApiError> {
        tx.execute(
            "INSERT INTO users (username, hash) VALUES (?1, ?2)",
            params![self.username, hash(&self.password, DEFAULT_COST)?],
        )?;

        let permissions = permissions_iter.into_iter().collect::<Vec<_>>();
        if permissions.is_empty() {
            return Ok(());
        }

        let placeholders = permissions
            .iter()
            .map(|_| "(?, ?)")
            .collect::<Vec<_>>()
            .join(", ");

        let params = params_from_iter(
            permissions
                .into_iter()
                .flat_map(|p| [<&'static str>::from(p), &self.username]),
        );

        let sql = format!(
            "INSERT INTO user_permissions (id, username) VALUES {}",
            placeholders
        );

        tx.execute(&sql, params)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub permissions: Vec<Permission>,
}

impl User {
    pub fn try_from_row(row: &Row) -> Result<Self, Error> {
        let permissions: Vec<Permission> = permissions_from_row(row)?;
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
        let db = match request.guard::<MyDatabase>().await {
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

        match db
            .run(move |conn| {
                conn.query_row(
                    "SELECT users.username AS username, COALESCE(GROUP_CONCAT(DISTINCT user_permissions.id), '') AS permissions FROM tokens
                    LEFT JOIN users ON tokens.username = users.username
                    LEFT JOIN user_permissions ON tokens.username = user_permissions.username
                    WHERE tokens.id = ?
                    GROUP BY user_permissions.username",
                    params![token],
                    User::try_from_row,
                )
            })
            .await
        {
            Ok(v) => Outcome::Success(v),
            Err(Error::QueryReturnedNoRows) => Outcome::Forward(Status::Unauthorized),
            Err(e) => Outcome::Error((Status::InternalServerError, ApiError::from(e))),
        }
    }
}
