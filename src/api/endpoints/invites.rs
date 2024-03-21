use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, params_from_iter, ToSql};

use crate::{
    api::data::{
        invites::Invite,
        permissions::{permissions_from_row, Permission},
        users::{DangerousLogin, User},
    },
    api::errors::ApiError,
    database::Database,
};

type Result<T> = std::result::Result<T, ApiError>;

// creates a new account
#[post("/invite/<code>", data = "<login>")]
async fn invite_use(db: Database, code: String, login: Json<DangerousLogin>) -> Result<()> {
    let login = login.into_inner();

    db.run(move |conn| -> Result<()> {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?; // delete run when the invite remaining = 0

        let tx = conn.transaction()?;

        let (remaining, permissions): (u16, Vec<Permission>) = tx
            .query_row(
                "SELECT invites.remaining AS remaining, GROUP_CONCAT(DISTINCT invite_permissions.id) AS permissions FROM invites
                LEFT JOIN invite_permissions ON invites.code = invite_permissions.code
                WHERE invites.code = ?
                GROUP BY invites.code",
                params![code],
                |row| Ok((row.get("remaining")?, permissions_from_row(row)?)),
            )
            .map_err(|e| ApiError::from(e))?;

        login.insert_user_into_transaction(permissions, &tx)?;

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
async fn invite_write(db: Database, user: User, invite: Json<Invite>) -> Result<Json<Invite>> {
    let mut invite = invite.into_inner();
    let mut required_permissions = invite.permissions.to_owned();
    required_permissions.push(Permission::InviteWrite);

    if !required_permissions
        .iter()
        .all(|permission| user.permissions.contains(permission))
    {
        Err(Status::Forbidden)?
    }

    invite.creator = user.username;
    db.run(move |conn| -> Result<Json<Invite>> {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let tx = conn.transaction()?;

        if tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM invites WHERE code = ?)",
            params![invite.code],
            |row| Ok(row.get::<usize, u8>(0)? == 1),
        )? {
            Err(Status::Conflict)?
        }

        tx.execute(
            "INSERT INTO invites (code, remaining, creator) VALUES (?1, ?2, ?3)",
            params![invite.code, invite.remaining, invite.creator],
        )?;

        // idk why you would want a invite with no permissions...
        if invite.permissions.is_empty() {
            // don't forget to commit, I mean its a bit late but...
            tx.commit()?;
            return Ok(Json(invite));
        }

        let sql = format!(
            "INSERT INTO invite_permissions (id, code) VALUES {}",
            invite
                .permissions
                .iter()
                .map(|_| "(?, ?)".to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        let params = params_from_iter(
            invite
                .permissions
                .iter()
                .flat_map(|p| [<&'static str>::from(p), &invite.code]),
        );

        tx.execute(&sql, params)?;

        tx.commit()?;

        Ok(Json(invite))
    })
    .await
}

#[get("/invite?<code>&<permissions>&<maxremaining>&<minremaining>&<creator>&<limit>")]
async fn invite_get(
    db: Database,
    user: User,
    code: Option<String>,
    permissions: Option<Json<Vec<Permission>>>,
    maxremaining: Option<u16>,
    minremaining: Option<u16>,
    creator: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<Invite>>> {
    if !user.permissions.contains(&Permission::InviteRead) {
        return Err(Status::Forbidden)?;
    }

    db.run(move |conn| -> Result<Json<Vec<Invite>>> {
        let mut sql = "SELECT invites.code, GROUP_CONCAT(DISTINCT invite_permissions.id) AS permissions, invites.remaining, invites.creator FROM invites
        LEFT JOIN invite_permissions ON invites.code = invite_permissions.code
        WHERE 1=1".to_string();
        let mut params_vec = vec![];

        if let Some(code_val) = code {
            sql += " AND invites.code LIKE ?";
            params_vec.push(format!("%{}%", code_val));
        }
        if let Some(permissions_val) = permissions {
            let permissions_val = permissions_val.into_inner();
            sql += &format!(" AND invite_permissions.id IN ({})", permissions_val.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));
            params_vec.extend(permissions_val.into_iter().map(|p| p.to_string()));
        }
        if let Some(maxremaining_val) = maxremaining {
            sql += " AND invites.remaining <= ?";
            params_vec.push(maxremaining_val.to_string());
        }
        if let Some(minremaining_val) = minremaining {
            sql += " AND invites.remaining >= ?";
            params_vec.push(minremaining_val.to_string());
        }
        if let Some(creator_val) = creator {
            sql += " AND invites.creator LIKE ?";
            params_vec.push(format!("%{}%", creator_val));
        }

        sql += " GROUP BY invites.code";

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
async fn invite_delete(db: Database, user: User, code: String) -> Result<()> {
    if !user.permissions.contains(&Permission::InviteDelete) {
        return Err(Status::Forbidden)?;
    }

    db.run(move |conn| {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        conn.execute("DELETE FROM invites WHERE code = ?", params![code])
    })
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
