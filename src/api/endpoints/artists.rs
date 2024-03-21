use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows, ToSql};

use crate::{
    api::errors::ApiError,
    database::Database, 
    api::data::{artists::Artist, permissions::Permission, users::User},
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/artist", data = "<artist>")]
async fn artist_write(db: Database, user: User, artist: Json<Artist>) -> Result<Json<Artist>> {
    if !user.permissions.contains(&Permission::ArtistWrite) {
        Err(Status::Forbidden)?
    }

    let artist = artist.into_inner();

    db.run(move |conn| -> Result<Json<Artist>> {
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO artists (id, name, bio) VALUES (?1, ?2, ?3)",
            params![artist.id, artist.name, artist.bio],
        )?;

        for genre in artist.genres.iter() {
            let genre_id: String = tx
                .query_row(
                    "SELECT id FROM genres WHERE id = ?",
                    params![genre],
                    |row| row.get(0),
                )
                .map_err(|e| match e {
                    QueryReturnedNoRows => {
                        ApiError::RusqliteError((Status::BadRequest, "Genre Not Found".to_string()))
                    }
                    e => ApiError::from(e),
                })?;
            tx.execute(
                "INSERT INTO artist_genres (artist_id, genre_id) VALUES (?1, ?2)",
                params![artist.id, genre_id],
            )?;
        }

        tx.commit()?;

        Ok(Json(artist))
    })
    .await
}

#[get("/artist?<id>&<name>&<genres>&<limit>")]
async fn artist_get(
    db: Database,
    user: User,
    id: Option<String>,
    name: Option<String>,
    genres: Option<Json<Vec<String>>>,
    limit: Option<u16>,
) -> Result<Json<Vec<Artist>>> {
    if !user.permissions.contains(&Permission::ArtistRead) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<Artist>>> {
        let mut sql = "SELECT artists.id, artists.name, artists.bio, COALESCE(GROUP_CONCAT(artist_genres.genre_id), '') AS genres FROM artists
            LEFT JOIN artist_genres ON artists.id = artist_genres.artist_id WHERE 1=1".to_string();
        let mut params_vec = Vec::new();

        if let Some(id_val) = id {
            sql += " AND artists.id = ?";
            params_vec.push(id_val);
        }

        if let Some(name_val) = name {
            sql += " AND artists.name LIKE ?";
            params_vec.push(format!("%{}%", name_val));
        }

        if let Some(genres_val) = genres {
            let genres_val = genres_val.into_inner();
            let genre_placeholders = genres_val.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            sql += &format!(" AND artist_genres.genre_id IN ({})", genre_placeholders);
            params_vec.extend(genres_val);
        }

        sql += &format!(" GROUP BY artists.id LIMIT {}", limit.unwrap_or(50));

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], |row| {
                    let genres: String = row.get(3)?;
                    let genres_vec: Vec<String> = genres.split(',').map(|s| s.to_string()).collect();
                    Ok(Artist {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        bio: row.get(2)?,
                        genres: genres_vec,
                    })
                })?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<Artist>>>()?,
        ))
    })
    .await
}

#[delete("/artist/<id>")]
async fn artist_delete(db: Database, user: User, id: String) -> Result<()> {
    if !user.permissions.contains(&Permission::ArtistDelete) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let tx = conn.transaction()?;

        if let Err(QueryReturnedNoRows) = tx.query_row(
            "SELECT 1 FROM artists WHERE id = ?",
            params![id],
            |_| Ok(()),
        ) {
            Err(Status::NotFound)?
        }

        tx.execute("DELETE FROM artists WHERE id = ?", params![id])?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Artist EndPoints", |rocket| async {
        rocket.mount("/", routes![artist_write, artist_get, artist_delete])
    })
}
