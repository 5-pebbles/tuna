use rocket::{fairing::AdHoc, http::Status};
use rocket_sync_db_pools::rusqlite::params;

use crate::{
    api::errors::ApiError,
    database::{database::Database, permissions::Permission, users::DangerousUser},
};

#[delete("/session/<username>")]
async fn sessions_delete(
    db: Database,
    user: DangerousUser,
    username: String,
) -> Result<(), ApiError> {
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
        rocket.mount("/", routes![sessions_delete])
    })
}
