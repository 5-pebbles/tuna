use bcrypt::verify;
use rocket::{fairing::AdHoc, http::{Status, CookieJar}, serde::json::Json};
use rocket_sync_db_pools::rusqlite::params;
use uuid::Uuid;
use sqlvec::SqlVec;

use crate::{
    api::errors::ApiError,
    database::{database::Database, permissions::Permission, users::{DangerousUser, DangerousLogin}},
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/login", data = "<login>")]
async fn session_write(
    db: Database,
    jar: &CookieJar<'_>,
    login: Json<DangerousLogin>,
) -> Result<()> {
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

#[delete("/session/<username>")]
async fn session_delete(
    db: Database,
    user: DangerousUser,
    username: String,
) -> Result<()> {
    if username != user.username && !user.has_permissions(&[Permission::SessionsDelete]) {
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
    AdHoc::on_ignite("API Session EndPoints", |rocket| async {
        rocket.mount("/", routes![session_write, session_delete])
    })
}
