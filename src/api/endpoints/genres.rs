use crate::{
    error::ApiError,
    database::MyDatabase, 
    api::data::{permissions::Permission, users::User},
};
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows, ToSql};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/genre/<genre>")]
async fn genre_write(db: MyDatabase, user: User, genre: String) -> Result<Json<String>> {
    if !user.permissions.contains(&Permission::GenreWrite) {
        Err(Status::Forbidden)?
    }
    db.run(move |conn| -> Result<Json<String>> {
        conn.execute("INSERT INTO genres (id) VALUES (?1)", params![genre])?;

        Ok(Json(genre))
    })
    .await
}

#[get("/genre?<genre>&<limit>")]
async fn genre_get(
    db: MyDatabase,
    user: User,
    genre: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<String>>> {
    if !user.permissions.contains(&Permission::GenreRead) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<String>>> {
        let mut sql = "SELECT * FROM genres WHERE 1=1".to_string();
        let mut params_vec = Vec::new();

        if let Some(genre_val) = genre {
            sql += " AND id LIKE ?";
            params_vec.push(format!("%{}%", genre_val));
        }

        sql += &format!(" LIMIT {}", limit.unwrap_or(50));

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], |row| row.get(0))?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<String>>>()?,
        ))
    })
    .await
}

/// Delete a genre from the database.
///
/// Requires: `GenreDelete` permission.
#[utoipa::path(
    responses(
        (status = 200, description = "Success"),
        (status = 403, description = "Forbidden requires permission `GenreDelete`"),
        (status = 404, description = "Not Found genre does not exist"),
    ),
    params(
        ("genre" = String, description = "The genre to be deleted")
    ),
    security(
        ("permissions" = ["GenreDelete"])
    ),
)]
#[delete("/genre/<genre>")]
async fn genre_delete(db: MyDatabase, user: User, genre: String) -> Result<()> {
    if !user.permissions.contains(&Permission::GenreDelete) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        if let Err(QueryReturnedNoRows) =
            tx.query_row("SELECT 1 FROM genres WHERE id = ?", params![genre], |_| {
                Ok(())
            })
        {
            Err(Status::NotFound)?
        }

        tx.execute("DELETE FROM genres WHERE id = ?", params![genre])?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Genre EndPoints", |rocket| async {
        rocket.mount("/", routes![genre_write, genre_get, genre_delete])
    })
}
