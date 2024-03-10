use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::{fairing::AdHoc, http::CookieJar, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, ToSql};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{
    api::{errors::ApiError, user::UserApiItem},
    database::{
        database::Database,
        permissions::Permission,
        users::{DangerousLogin, DangerousUser},
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

#[post("/login", data = "<login>")]
async fn user_login(db: Database, jar: &CookieJar<'_>, login: Json<DangerousLogin>) -> Result<()> {
    let login = login.into_inner();
    let session: String = db
        .run(move |conn| -> Result<String> {
            let tx = conn.transaction()?;

            let (hash, mut sessions): (String, Vec<String>) = tx.query_row(
                "SELECT hash, sessions FROM users WHERE username = ?",
                params![&login.username],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get::<usize, SqlVec<String>>(1)?.into_inner(),
                    ))
                },
            )?;

            if !verify(login.password, &hash)? {
                Err(Status::Forbidden)?
            }

            let session: String = Uuid::new_v4().to_string();
            sessions.push(session.clone());

            tx.execute(
                "UPDATE users SET sessions = ? WHERE username = ?",
                params![SqlVec::new(sessions), login.username],
            )?;

            tx.commit()?;

            Ok(session)
        })
        .await?;
    jar.add(("session", session));
    Ok(())
}

#[get("/user?<username>&<permissions>&<limit>")]
async fn user_get(
    db: Database,
    user: DangerousUser,
    username: Option<String>,
    permissions: Option<Json<Vec<Permission>>>,
    limit: Option<u16>,
) -> Result<Json<Vec<UserApiItem>>> {
    if !user.has_permissions(&[Permission::UserRead]) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<UserApiItem>>> {
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
                .query_map(&params_sql[..], UserApiItem::try_from_row)?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<UserApiItem>>>()?,
        ))
    })
    .await
}

#[delete("/user/<username>")]
async fn user_delete(db: Database, user: DangerousUser, username: &str) -> Result<()> {
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

            if !user.has_permissions(&required_permissions) {
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
        rocket.mount("/", routes![user_init, user_login, user_get, user_delete,])
    })
}
