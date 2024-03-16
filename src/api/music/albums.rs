use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows, ToSql};

use crate::{
    api::errors::ApiError,
    database::{albums::Album, database::Database, permissions::Permission, users::User},
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/album", data = "<album>")]
async fn album_write(db: Database, user: User, album: Json<Album>) -> Result<Json<Album>> {
    if !user.permissions.contains(&Permission::AlbumWrite) {
        Err(Status::Forbidden)?
    }

    let album = album.into_inner();

    db.run(move |conn| -> Result<Json<Album>> {
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO albums (id, name, release) VALUES (?1, ?2, ?3)",
            params![album.id, album.name, album.release],
        )?;

        for artist in album.artists.iter() {
            tx.execute(
                "INSERT INTO artist_albums (artist_id, album_id) VALUES (?1, ?2)",
                params![artist, album.id],
            )?;
        }

        for track in album.tracks.iter() {
            tx.execute(
                "INSERT INTO album_tracks (album_id, track_id) VALUES (?1, ?2)",
                params![album.id, track],
            )?;
        }

        for genre in album.genres.iter() {
            tx.execute(
                "INSERT INTO album_genres (album_id, genre_id) VALUES (?1, ?2)",
                params![album.id, genre],
            )?;
        }

        tx.commit()?;

        Ok(Json(album))
    })
    .await
}

#[get("/album?<id>&<name>&<maxrelease>&<minrelease>&<genres>&<maxcount>&<mincount>&<limit>")]
async fn album_get(
    db: Database,
    user: User,
    id: Option<String>,
    name: Option<String>,
    maxrelease: Option<u16>,
    minrelease: Option<u16>,
    genres: Option<Json<Vec<String>>>,
    maxcount: Option<u16>,
    mincount: Option<u16>,
    limit: Option<u16>,
) -> Result<Json<Vec<Album>>> {
    if !user.permissions.contains(&Permission::AlbumRead) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<Json<Vec<Album>>> {
        let mut sql = "SELECT albums.id, albums.name, albums.release, COALESCE(GROUP_CONCAT(DISTINCT artist_albums.artist_id), ''), COALESCE(GROUP_CONCAT(DISTINCT album_tracks.track_id), ''), COALESCE(GROUP_CONCAT(DISTINCT album_genres.genre_id), '') AS genres FROM albums 
            LEFT JOIN album_tracks ON albums.id = album_tracks.album_id
            LEFT JOIN artist_albums ON albums.id = artist_albums.album_id
            LEFT JOIN album_genres ON albums.id = album_genres.album_id WHERE 1=1".to_string();
        let mut params_vec = Vec::new();

        if let Some(id_val) = id {
            sql += " AND albums.id = ?";
            params_vec.push(id_val);
        }

        if let Some(name_val) = name {
            sql += " AND albums.name LIKE ?";
            params_vec.push(format!("%{}%", name_val));
        }

        if let Some(minrelease_val) = minrelease {
            sql += " AND albums.release >= ?";
            params_vec.push(minrelease_val.to_string());
        }

        if let Some(maxrelease_val) = maxrelease {
            sql += " AND albums.release <= ?";
            params_vec.push(maxrelease_val.to_string());
        }

        if let Some(maxcount_val) = maxcount {
            sql += " AND (SELECT COUNT(*) FROM album_tracks WHERE album_tracks.album_id = albums.id) <= ?";
            params_vec.push(maxcount_val.to_string());
        }

        if let Some(mincount_val) = mincount {
            sql += " AND (SELECT COUNT(*) FROM album_tracks WHERE album_tracks.album_id = albums.id) >= ?";
            params_vec.push(mincount_val.to_string());
        }

        if let Some(genres_val) = genres {
            let genres_val = genres_val.into_inner();
            let genre_placeholders = genres_val.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            sql += &format!(" AND album_genres.genre_id IN ({})", genre_placeholders);
            params_vec.extend(genres_val);
        }

        sql += &format!(" GROUP BY albums.id LIMIT {}", limit.unwrap_or(50));

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
            conn.prepare(&sql)?
                .query_map(&params_sql[..], |row| {

                    let artists_str: String = row.get(3)?;
                    let artists: Vec<String> = artists_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    let tracks_str: String = row.get(4)?;
                    let tracks: Vec<String> = tracks_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    let genres_str: String = row.get(5)?;
                    let genres: Vec<String> = genres_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    Ok(Album {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        release: row.get(2)?,
                        artists,
                        tracks,
                        genres,
                    })
                })?
                .map(|v| v.map_err(|e| ApiError::from(e)))
                .collect::<Result<Vec<Album>>>()?,
        ))
    })
    .await
}

#[delete("/album/<id>")]
async fn album_delete(db: Database, user: User, id: String) -> Result<()> {
    if !user.permissions.contains(&Permission::AlbumDelete) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;

        let tx = conn.transaction()?;

        if let Err(QueryReturnedNoRows) =
            tx.query_row("SELECT 1 FROM albums WHERE id = ?", params![id], |_| Ok(()))
        {
            Err(Status::NotFound)?
        }

        tx.execute("DELETE FROM albums WHERE id = ?", params![id])?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Album EndPoints", |rocket| async {
        rocket.mount("/", routes![album_write, album_get, album_delete])
    })
}
