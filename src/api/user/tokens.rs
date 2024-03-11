use bcrypt::verify;
use rocket::{
    fairing::AdHoc,
    http::{CookieJar, Status},
    serde::json::Json,
};
use rocket_sync_db_pools::rusqlite::params;
use sqlvec::SqlVec;
use uuid::Uuid;

use crate::{
    api::errors::ApiError,
    database::{
        database::Database,
        permissions::Permission,
        users::{DangerousLogin, DangerousUser},
    },
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/token", data = "<login>")]
async fn token_write(
    db: Database,
    jar: &CookieJar<'_>,
    login: Json<DangerousLogin>,
) -> Result<Json<String>> {
    let login = login.into_inner();
    let token: String = db
        .run(move |conn| -> Result<String> {
            let tx = conn.transaction()?;

            let (hash, mut tokens): (String, Vec<String>) = tx.query_row(
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

            let token: String = Uuid::new_v4().to_string();
            tokens.push(token.clone());

            tx.execute(
                "UPDATE users SET sessions = ? WHERE username = ?",
                params![SqlVec::new(tokens), login.username],
            )?;

            tx.commit()?;

            Ok(token)
        })
        .await?;
    jar.add(("token", token.clone()));
    Ok(Json(token))
}

#[delete("/token/<username>")]
async fn token_delete(db: Database, user: DangerousUser, username: String) -> Result<()> {
    if username != user.username && !user.has_permissions(&[Permission::TokenDelete]) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| {
        conn.execute(
            "UPDATE users SET sessions = '' WHERE username = ?",
            params![username],
        )
    })
    .await?;
    Ok(())
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Token EndPoints", |rocket| async {
        rocket.mount("/", routes![token_write, token_delete])
    })
}
