use crate::{
    api::errors::ApiError,
    database::{database::Database, permissions::Permission, users::DangerousUser},
};
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows, ToSql};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/genre/<genre>")]
async fn genre_write(db: Database, user: DangerousUser, genre: String) -> Result<Json<String>> {
    if !user.has_permissions(&[Permission::GenreWrite]) {
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
    db: Database,
    user: DangerousUser,
    genre: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<String>>> {
    if !user.has_permissions(&[Permission::GenreRead]) {
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

#[delete("/genre/<genre>")]
async fn genre_delete(db: Database, user: DangerousUser, genre: String) -> Result<()> {
    if !user.has_permissions(&[Permission::GenreDelete]) {
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
        tx.execute(
            "DELETE FROM artist_genres WHERE genre_id = ?",
            params![genre],
        )?;
        tx.execute(
            "DELETE FROM album_genres WHERE genre_id = ?",
            params![genre],
        )?;
        tx.execute(
            "DELETE FROM track_genres WHERE genre_id = ?",
            params![genre],
        )?;

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
