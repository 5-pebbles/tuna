use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, params_from_iter};
use strum::IntoEnumIterator;

use crate::{
    api::data::{
        permissions::{permissions_from_row, Permission},
        users::{DangerousLogin, User},
    },
    database::MyDatabase,
    error::ApiError,
};

type Result<T> = std::result::Result<T, ApiError>;

/// Creates the first user in the database.
///
/// This endpoint only works if the database is empty.
/// It allows the creation of the first user, who can then invite all other users.
/// The first user has all permissions available.
#[utoipa::path(
    request_body(
        content = DangerousLogin,
        description = "The username & password of the first user",
    ),
    responses(
        (status = 200, description = "The user was created successfully"),
        (status = 409, description = "Conflict the database is not empty"),
    ),
)]
#[post("/init", data = "<login>")]
async fn user_init(db: MyDatabase, login: Json<DangerousLogin>) -> Result<()> {
    let login = login.into_inner();

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        // if they are any rows in the db
        if tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM users) OR EXISTS(SELECT 1 FROM invites)",
            [],
            |row| Ok(row.get::<usize, u8>(0)? == 1),
        )? {
            Err(Status::Conflict)?
        };

        login.insert_user_into_transaction(Permission::iter(), &tx)?;

        tx.commit()?;

        Ok(())
    })
    .await
}

/// Retrieve a list of users.
///
/// Requires: `UserRead` permission.
#[utoipa::path(
    responses(
        (
            status = 200,
            description = "Success",
            body = Vec<User>,
        ),
        (
            status = 403,
            description = "Forbidden requires permission `UserRead`"
        )
    ),
    params(
        ("username", Query, description = "The username to search for"),
        ("permissions", Query, description = "The permissions the user must possess"),
        ("limit", Query, description = "The maximum number of users to return"),
    ),
    security(
        ("permissions" = ["UserRead"])
    ),
)]
#[get("/user?<username>&<permissions>&<limit>")]
async fn user_get(
    db: MyDatabase,
    user: User,
    username: Option<String>,
    permissions: Option<Json<Vec<Permission>>>,
    limit: Option<u16>,
) -> Result<Json<Vec<User>>> {
    if !user.permissions.contains(&Permission::UserRead) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<User>>> {
        let mut sql = "SELECT user_permissions.username AS username, COALESCE(GROUP_CONCAT(DISTINCT user_permissions.id), '') AS permissions
        FROM users
        LEFT JOIN user_permissions ON users.username = user_permissions.username
        WHERE 1=1".to_string();
        let mut params = Vec::new();

        if let Some(username_val) = username {
            sql += " AND users.username LIKE ?";
            params.push(format!("%{}%", username_val));
        }
        if let Some(permissions_val) = permissions {
            let permissions_val = permissions_val.into_inner();
            sql += &format!(" AND user_permissions.id CONTAINS ({})", permissions_val.iter().map(|_| "?".to_string()).collect::<Vec<String>>().join(", "));
            params.extend(permissions_val.into_iter().map(|p| p.to_string()));
        }

        sql += " GROUP BY user_permissions.username";
        if let Some(limit_val) = limit {
            sql += &format!(" LIMIT {}", limit_val)
        }

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(params_from_iter(params), User::try_from_row)?
                .map(|v| v.map_err(ApiError::from))
                .collect::<Result<Vec<User>>>()?,
        ))
    })
    .await
}

/// Deletes a user from the database.
///
/// Requires: `UserDelete` permission to delete another user, but you are free to delete yourself.
#[utoipa::path(
    responses(
        (status = 200, description = "Success"),
        (status = 403, description = "Forbidded you do not have the required permissions"),
    ),
    params(
        ("username", description = "The username of the user to delete"),
    ),
    security(
        ("permissions" = ["UserDelete"])
    ),
)]
#[delete("/user/<username>")]
async fn user_delete(db: MyDatabase, user: User, username: &str) -> Result<()> {
    let username = username.to_string(); // Fix Message: Using `String` as a parameter type is inefficient. Use `&str` instead.
    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        if username != user.username {
            // we cant select directly from the user_permissions table because the user might not have any permissions
            let mut required_permissions = tx.query_row(
                "SELECT GROUP_CONCAT(DISTINCT user_permissions.id) AS permissions FROM users
                    LEFT JOIN user_permissions ON users.username = user_permissions.username
                    WHERE users.username = ? GROUP BY users.username",
                params![username],
                permissions_from_row,
            )?;

            required_permissions.push(Permission::UserDelete);

            if !required_permissions
                .iter()
                .all(|permission| user.permissions.contains(permission))
            {
                Err(Status::Forbidden)?
            }
        }

        tx.execute("DELETE FROM users WHERE username = ?", params![username])?;

        tx.commit()?;
        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API User EndPoints", |rocket| async {
        rocket.mount("/", routes![user_init, user_get, user_delete,])
    })
}
