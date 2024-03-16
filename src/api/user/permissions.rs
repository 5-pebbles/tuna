use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, params_from_iter};

use std::str::FromStr;

use crate::{
    api::errors::ApiError,
    database::{database::Database, permissions::{Permission, permissions_from_row}, users::User},
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/permissions/<username>", data = "<permissions_to_add>")]
async fn permissions_add(
    db: Database,
    user: User,
    username: String,
    permissions_to_add: Json<Vec<Permission>>,
) -> Result<()> {
    let permissions_to_add = permissions_to_add.into_inner();

    let mut required_permissions = permissions_to_add.clone();
    required_permissions.push(Permission::PermissionAdd);
    if !required_permissions
        .iter()
        .all(|permission| user.permissions.contains(permission))
    {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        let sql = format!(
            "INSERT OR IGNORE INTO user_permissions (id, username) VALUES {};",
            permissions_to_add.iter().map(|_| format!("\n (?, ?)"))
                .collect::<Vec<String>>()
                .join(",")
        );

        let params = params_from_iter(permissions_to_add.into_iter()
            .map(|p| [<&'static str>::from(p), &username]).flatten());

        tx.execute(
            &sql,
            params,
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
    user: User,
    username: String,
    permissions_to_delete: Json<Vec<Permission>>,
) -> Result<()> {
    let permissions_to_delete = permissions_to_delete.into_inner();

    db.run(move |conn| {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let tx = conn.transaction()?;
        
        let mut required_permissions = tx.query_row(
            "SELECT GROUP_CONCAT(DISTINCT id) AS permissions FROM user_permissions
            WHERE username = ?",
            params![&username],
            permissions_from_row,
            )?;


        required_permissions.push(Permission::PermissionDelete);
        if !required_permissions
            .iter()
            .all(|permission| user.permissions.contains(permission))
        {
            Err(Status::Forbidden)?
        }


        let permission_placeholders = permissions_to_delete.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

        let params = params_from_iter(
            std::iter::once(username.as_str())
            .chain(
                permissions_to_delete
                .into_iter()
                .map(|p| <&'static str>::from(p))
            )
        );

        tx.execute(
            &format!("DELETE FROM user_permissions WHERE username = ? AND id IN ({})", permission_placeholders),
            params,
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
