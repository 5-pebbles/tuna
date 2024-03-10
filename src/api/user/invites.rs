use bcrypt::{hash, DEFAULT_COST};
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, ToSql};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

use crate::{
    api::errors::ApiError,
    database::{
        database::Database,
        invites::Invite,
        permissions::Permission,
        users::{DangerousLogin, DangerousUser},
    },
};

type Result<T> = std::result::Result<T, ApiError>;

// creates a new account
#[post("/invite/<code>", data = "<login>")]
async fn invite_use(db: Database, code: String, login: Json<DangerousLogin>) -> Result<()> {
    let login = login.into_inner();

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        let (permissions, remaining): (SqlVec<Permission>, u16) = tx
            .query_row(
                "SELECT permissions, remaining FROM invites WHERE code = ?",
                params![code],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|e| ApiError::from(e))?;

        tx.execute(
            "INSERT INTO users (username, permissions, hash, sessions) VALUES (?1, ?2, ?3, ?4)",
            params![
                login.username,
                permissions,
                hash(login.password, DEFAULT_COST)?,
                ""
            ],
        )?;

        if remaining > 1 {
            tx.execute(
                "UPDATE invites SET remaining = ? WHERE code = ?",
                params![remaining - 1, code],
            )?;
        } else {
            tx.execute("DELETE FROM invites WHERE code = ?", params![code])?;
        }
        tx.commit()?;
        Ok(())
    })
    .await
}

#[post("/invite", data = "<invite>")]
async fn invite_write(
    db: Database,
    user: DangerousUser,
    invite: Json<Invite>,
) -> Result<Json<Invite>> {
    let mut invite = invite.into_inner();
    let mut required_permissions = invite.permissions.inner().to_owned();
    required_permissions.push(Permission::InviteWrite);
    if !user.has_permissions(&required_permissions) {
        Err(Status::Forbidden)?
    }

    invite.creator = user.username;
    db.run(move |conn| -> Result<Json<Invite>> {
        if conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM invites WHERE code = ?)",
            params![invite.code],
            |row| Ok(row.get::<usize, u8>(0)? == 1),
        )? {
            Err(Status::Conflict)?
        }

        conn.execute(
            "INSERT INTO invites (code, permissions, remaining, creator) VALUES (?1, ?2, ?3, ?4)",
            params![
                invite.code,
                invite.permissions,
                invite.remaining,
                invite.creator
            ],
        )?;
        Ok(Json(invite))
    })
    .await
}

#[get("/invite?<code>&<permissions>&<maxremaining>&<minremaining>&<creator>&<limit>")]
async fn invite_get(
    db: Database,
    user: DangerousUser,
    code: Option<String>,
    permissions: Option<Json<Vec<Permission>>>,
    maxremaining: Option<u16>,
    minremaining: Option<u16>,
    creator: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<Invite>>> {
    if !user.has_permissions(&[Permission::InviteRead]) {
        return Err(Status::Forbidden)?;
    }

    db.run(move |conn| -> Result<Json<Vec<Invite>>> {
        let mut sql = "SELECT * FROM invites WHERE 1=1".to_string();
        let mut params_vec = vec![];

        if let Some(code_val) = code {
            sql += " AND code LIKE ?";
            params_vec.push(format!("%{}%", code_val));
        }
        if let Some(permissions_val) = permissions {
            sql += " AND permissions LIKE ?";
            params_vec.push(format!(
                "%{}%",
                SqlVec::new(permissions_val.into_inner()).to_string()
            ));
        }
        if let Some(maxremaining_val) = maxremaining {
            sql += " AND remaining <= ?";
            params_vec.push(maxremaining_val.to_string());
        }
        if let Some(minremaining_val) = minremaining {
            sql += " AND remaining >= ?";
            params_vec.push(minremaining_val.to_string());
        }
        if let Some(creator_val) = creator {
            sql += " AND creator LIKE ?";
            params_vec.push(format!("%{}%", creator_val));
        }

        if let Some(limit_val) = limit {
            sql += &format!(" LIMIT {}", limit_val);
        }

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], Invite::try_from_row)?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<Invite>>>()?,
        ))
    })
    .await
}

#[delete("/invite/<code>")]
async fn invite_delete(db: Database, user: DangerousUser, code: String) -> Result<()> {
    if !user.has_permissions(&[Permission::InviteDelete]) {
        return Err(Status::Forbidden)?;
    }

    db.run(move |conn| conn.execute("DELETE FROM invites WHERE code = ?", params![code]))
        .await?;
    Ok(())
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Invite EndPoints", |rocket| async {
        rocket.mount(
            "/",
            routes![invite_use, invite_write, invite_get, invite_delete],
        )
    })
}
