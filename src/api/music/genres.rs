use crate::{
    api::errors::ApiError,
    database::{database::Database, genres::Genre, permissions::Permission, users::DangerousUser},
};
use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, ToSql};
use rusqlite_from_row::FromRow;

type Result<T> = std::result::Result<T, ApiError>;

#[post("/genre", data = "<genre>")]
async fn genre_write(db: Database, user: DangerousUser, genre: Json<Genre>) -> Result<Json<Genre>> {
    if !user.has_permissions(&[Permission::GenreWrite]) {
        Err(Status::Forbidden)?
    }
    let genre = genre.into_inner();
    db.run(move |conn| -> Result<Json<Genre>> {
        conn.execute(
            "INSERT INTO genres (id, name) VALUES (?1, ?2)",
            params![genre.id, genre.name],
        )?;

        Ok(Json(genre))
    })
    .await
}

#[get("/genre?<id>&<name>&<limit>")]
async fn genre_get(
    db: Database,
    user: DangerousUser,
    id: Option<u16>,
    name: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<Genre>>> {
    if !user.has_permissions(&[Permission::GenreRead]) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<Genre>>> {
        let mut sql = "SELECT * FROM genres WHERE 1=1".to_string();
        let mut params_vec = Vec::new();

        if let Some(id_val) = id {
            sql += " AND id = ?";
            params_vec.push(id_val.to_string());
        }

        if let Some(name_val) = name {
            sql += " AND name LIKE ?";
            params_vec.push(format!("%{}%", name_val));
        }

        sql += &format!(" LIMIT {}", limit.unwrap_or(50));

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], Genre::try_from_row)?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<Genre>>>()?,
        ))
    })
    .await
}

#[delete("/genre/<id>")]
async fn genre_delete(db: Database, user: DangerousUser, id: u16) -> Result<()> {
    if !user.has_permissions(&[Permission::GenreDelete]) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        conn.execute("DELETE FROM genres WHERE id = ?", params![id])?;
        conn.execute("DELETE FROM artist_genres WHERE genre_id = ?", params![id])?;
        conn.execute("DELETE FROM album_genres WHERE genre_id = ?", params![id])?;
        conn.execute("DELETE FROM track_genres WHERE genre_id = ?", params![id])?;
        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Genre EndPoints", |rocket| async {
        rocket.mount("/", routes![genre_write, genre_get, genre_delete])
    })
}
