use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, params_from_iter};

use crate::{
    api::data::{
        permissions::{permissions_from_row, Permission},
        users::User,
    },
    error::ApiError,
    database::MyDatabase,
};

type Result<T> = std::result::Result<T, ApiError>;

/// Grant another user a list of permissions
///
/// Requires: `PermissionAdd` as well as all permissions you intend to grant
#[utoipa::path(
    request_body(
        content = Vec<Permission>,
        description = "A list of permissions to grant",
        example = json!([Permission::DocsRead, Permission::PermissionAdd]),
    ),
    responses(
    (
        status = 200,
        description = "Success",
    ),
    (
        status = 403,
        description = "Forbidden you do not have the required permissions",
    )),
    params(
        ("username" = String, description = "The username of the user you would like to grant permissions to")
    ),
    security(
        ("permissions" = ["PermissionAdd"])
    ),
)]
#[post("/permission/<username>", data = "<permissions_to_add>")]
async fn permission_add(
    db: MyDatabase,
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
            permissions_to_add
                .iter()
                .map(|_| "\n (?, ?)".to_string())
                .collect::<Vec<String>>()
                .join(",")
        );

        let params = params_from_iter(
            permissions_to_add
                .into_iter()
                .flat_map(|p| [<&'static str>::from(p), &username]),
        );

        tx.execute(&sql, params)?;
        tx.commit()?;
        Ok(())
    })
    .await?;
    Ok(())
}

/// Revoke a list of permissions from a user
///
/// Requires: `PermissionDelete` & all permissions of the user who's permissions are being revoked
#[utoipa::path(
    request_body(
        content = Vec<Permission>,
        description = "A list of permissions to Revoke",
        example = json!([Permission::DocsRead, Permission::PermissionDelete]),
    ),
    responses(
    (
        status = 200,
        description = "Success",
    ),
    (
        status = 403,
        description = "Forbidden you do not have the required permissions",
    )),
    params(
        ("username" = String, description = "The username of the user who's permissions you would like to revoke")
    ),
    security(
        ("permissions" = ["PermissionDelete"])
    ),
)]
#[delete("/permission/<username>", data = "<permissions_to_delete>")]
async fn permission_delete(
    db: MyDatabase,
    user: User,
    username: String,
    permissions_to_delete: Json<Vec<Permission>>,
) -> Result<()> {
    let permissions_to_delete = permissions_to_delete.into_inner();

    db.run(move |conn| {
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

        let permission_placeholders = permissions_to_delete
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        // You can't fix the redundant closure here or you get a lifetime error
        // It feels like it should work the same but IDK
        #[allow(clippy::redundant_closure)]
        let params = params_from_iter(
            std::iter::once(username.as_str()).chain(
                permissions_to_delete
                    .into_iter()
                    .map(|p| <&'static str>::from(p)),
            ),
        );

        tx.execute(
            &format!(
                "DELETE FROM user_permissions WHERE username = ? AND id IN ({})",
                permission_placeholders
            ),
            params,
        )?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Permissions EndPoints", |rocket| async {
        rocket.mount("/", routes![permission_add, permission_delete,])
    })
}
