use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::params;
use sqlvec::SqlVec;

use crate::{
    api::errors::ApiError,
    database::{database::Database, permissions::Permission, users::DangerousUser},
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/permissions/<username>", data = "<permissions_to_add>")]
async fn permissions_add(
    db: Database,
    user: DangerousUser,
    username: String,
    permissions_to_add: Json<Vec<Permission>>,
) -> Result<()> {
    let permissions_to_add = permissions_to_add.into_inner();

    let mut required_permissions = permissions_to_add.clone();
    required_permissions.push(Permission::PermissionAdd);
    if !user.has_permissions(&required_permissions) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;
        dbg!(&permissions_to_add);
        let all_permissions = [
            permissions_to_add,
            tx.query_row(
                "SELECT permissions FROM users WHERE username = ?",
                params![&username],
                |row| row.get::<usize, SqlVec<Permission>>(0),
            )?
            .into_inner(),
        ]
        .concat();
        dbg!(&all_permissions);

        tx.execute(
            "UPDATE users SET permissions = ? WHERE username = ?",
            params![SqlVec::new(all_permissions), username],
        )?;
        tx.commit()?;
        Ok(())
    })
    .await?;
    Ok(())
}

#[delete("/permissions/<username>", data = "<permissions_to_delete>")]
async fn permissions_delete(
    db: Database,
    user: DangerousUser,
    username: String,
    permissions_to_delete: Json<Vec<Permission>>,
) -> Result<()> {
    let permissions_to_delete = permissions_to_delete.into_inner();

    db.run(move |conn| {
        let tx = conn.transaction()?;

        let mut current_permissions = tx
            .query_row(
                "SELECT permissions FROM users WHERE username = ?",
                params![&username],
                |row| Ok(row.get::<usize, SqlVec<Permission>>(0)?),
            )?
            .into_inner();

        let mut required_permissions = current_permissions.clone();
        required_permissions.push(Permission::PermissionDelete);
        if !user.has_permissions(&required_permissions) {
            Err(Status::Forbidden)?
        }

        current_permissions.retain(|v| !permissions_to_delete.contains(v));

        tx.execute(
            "UPDATE users SET permissions = ? WHERE username = ?",
            params![SqlVec::new(current_permissions), username],
        )?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Permissions EndPoints", |rocket| async {
        rocket.mount("/", routes![permissions_add, permissions_delete,])
    })
}
