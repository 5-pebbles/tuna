use bcrypt::verify;
use rocket::{
    fairing::AdHoc,
    http::{CookieJar, Status},
    serde::json::Json,
};
use rocket_sync_db_pools::rusqlite::params;
use uuid::Uuid;

use crate::{
    api::data::{
        permissions::Permission,
        users::{DangerousLogin, User},
    },
    database::MyDatabase,
    error::ApiError,
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/token", data = "<login>")]
async fn token_write(
    db: MyDatabase,
    jar: &CookieJar<'_>,
    login: Json<DangerousLogin>,
) -> Result<Json<String>> {
    let login = login.into_inner();
    let token: String = db
        .run(move |conn| -> Result<String> {
            let tx = conn.transaction()?;

            let hash: String = tx.query_row(
                "SELECT hash FROM users WHERE username = ?",
                params![&login.username],
                |row| row.get("hash"),
            )?;

            if !verify(login.password, &hash)? {
                Err(Status::Forbidden)?
            }

            let token: String = Uuid::new_v4().to_string();

            tx.execute(
                "INSERT INTO tokens (id, username) VALUES (?1, ?2)",
                params![token, login.username],
            )?;

            tx.commit()?;

            Ok(token)
        })
        .await?;
    jar.add(("token", token.clone()));
    Ok(Json(token))
}

#[delete("/token/<username>")]
async fn token_delete(db: MyDatabase, user: User, username: String) -> Result<()> {
    if username != user.username && !user.permissions.contains(&Permission::TokenDelete) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| conn.execute("DELETE FROM tokens WHERE username = ?", params![username]))
        .await?;
    Ok(())
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Token EndPoints", |rocket| async {
        rocket.mount("/", routes![token_write, token_delete])
    })
}
