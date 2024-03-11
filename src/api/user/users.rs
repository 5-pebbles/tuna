use bcrypt::{hash, DEFAULT_COST};
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, ToSql};
use strum::IntoEnumIterator;
use sqlvec::SqlVec;

use crate::{
    api::errors::ApiError,
    database::{
        database::Database,
        permissions::Permission,
        users::{DangerousLogin, User},
    },
};

type Result<T> = std::result::Result<T, ApiError>;

// create the first user in the database
#[post("/init", data = "<login>")]
async fn user_init(db: Database, login: Json<DangerousLogin>) -> Result<()> {
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

        tx.execute(
            "INSERT INTO users (username, hash, permissions) VALUES (?1, ?2, ?3)",
            params![
                login.username,
                hash(login.password, DEFAULT_COST)?,
                SqlVec::new(Permission::iter().collect::<Vec<Permission>>())
            ],
        )?;

        tx.commit()?;

        Ok(())
    })
    .await
}

#[get("/user?<username>&<permissions>&<limit>")]
async fn user_get(
    db: Database,
    user: User,
    username: Option<String>,
    permissions: Option<Json<Vec<Permission>>>,
    limit: Option<u16>,
) -> Result<Json<Vec<User>>> {
    if !user.permissions.contains(&Permission::UserRead) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<User>>> {
        let mut sql = "SELECT username, permissions FROM users WHERE 1=1".to_string();
        let mut params_vec = vec![];

        if let Some(username_val) = username {
            sql += " AND username LIKE ?";
            params_vec.push(format!("%{}%", username_val));
        }
        if let Some(permissions_val) = permissions {
            sql += " AND permissions LIKE ?";
            params_vec.push(format!(
                "%{}%",
                SqlVec::new(permissions_val.into_inner()).to_string()
            ));
        }
        if let Some(limit_val) = limit {
            sql += &format!(" LIMIT {}", limit_val)
        }

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], User::try_from_row)?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<User>>>()?,
        ))
    })
    .await
}

#[delete("/user/<username>")]
async fn user_delete(db: Database, user: User, username: &str) -> Result<()> {
    let username = username.to_string(); // Fix Message: Using `String` as a parameter type is inefficient. Use `&str` instead.
    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        if username != user.username {
            let mut required_permissions = tx
                .query_row(
                    "SELECT permissions FROM users WHERE username = ?",
                    params![username],
                    |row| row.get::<usize, SqlVec<Permission>>(0),
                )?
                .into_inner();

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
