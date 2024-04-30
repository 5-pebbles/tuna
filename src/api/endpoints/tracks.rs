use rocket::{fairing::AdHoc, http::Status, serde::json::Json};
use rocket_sync_db_pools::rusqlite::{params, Error::QueryReturnedNoRows, ToSql};

use crate::{
    api::data::{permissions::Permission, tracks::Track, users::User},
    database::MyDatabase,
    error::ApiError,
};

type Result<T> = std::result::Result<T, ApiError>;

#[post("/track", data = "<track>")]
async fn track_write(db: MyDatabase, user: User, track: Json<Track>) -> Result<Json<Track>> {
    if !user.permissions.contains(&Permission::TrackWrite) {
        Err(Status::Forbidden)?
    }

    let mut track = track.into_inner();

    db.run(move |conn| -> Result<Json<Track>> {
        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO tracks (id, name, release, lyrics) VALUES (?1, ?2, ?3, ?4)",
            params![track.id, track.name, track.release, track.lyrics,],
        )?;

        for album in track.albums.iter() {
            tx.execute(
                "INSERT INTO album_tracks (album_id, track_id) VALUES (?1, ?2)",
                params![album, track.id],
            )?;

            // return track should contain the track's artists
            track.artists.extend(
                tx.prepare("SELECT artist_id FROM artist_albums WHERE album_id = ?")?
                    .query_map(params![album], |row| row.get::<usize, String>(0))?
                    .map(|v| v.map_err(ApiError::from))
                    .collect::<Result<Vec<String>>>()?,
            );
        }

        for genre in track.genres.iter() {
            tx.execute(
                "INSERT INTO track_genres (track_id, genre_id) VALUES (?1, ?2)",
                params![track.id, genre],
            )?;
        }

        tx.commit()?;

        Ok(Json(track))
    })
    .await
}

#[get("/track?<id>&<name>&<maxrelease>&<minrelease>&<genres>&<albums>&<artists>&<lyrics>&<limit>")]
async fn track_get(
    db: MyDatabase,
    user: User,
    id: Option<String>,
    name: Option<String>,
    maxrelease: Option<u16>,
    minrelease: Option<u16>,
    genres: Option<Json<Vec<String>>>,
    albums: Option<Json<Vec<String>>>,
    artists: Option<Json<Vec<String>>>,
    lyrics: Option<String>,
    limit: Option<u16>,
) -> Result<Json<Vec<Track>>> {
    if !user.permissions.contains(&Permission::TrackRead) {
        Err(Status::Forbidden)?
    }
    db.run(move |conn| -> Result<Json<Vec<Track>>> {
        let mut sql = "SELECT id, name, release, duration, COALESCE(GROUP_CONCAT(DISTINCT album_tracks.album_id), '') AS albums, COALESCE(GROUP_CONCAT(DISTINCT artist_albums.artist_id), '') AS artists, lyrics, COALESCE(GROUP_CONCAT(DISTINCT track_genres.genre_id), '') AS genres FROM tracks
            LEFT JOIN track_genres ON tracks.id = track_genres.track_id
            LEFT JOIN album_tracks ON tracks.id = album_tracks.track_id
            LEFT JOIN artist_albums ON album_tracks.album_id = artist_albums.album_id WHERE 1=1".to_string();
        let mut params_vec = Vec::new();

        if let Some(id_val) = id {
            sql += " AND id = ?";
            params_vec.push(id_val);
        }

        if let Some(name_val) = name {
            sql += " AND name LIKE ?";
            params_vec.push(format!("%{}%", name_val));
        }

        if let Some(maxrelease_val) = maxrelease {
            sql += " AND release <= ?";
            params_vec.push(maxrelease_val.to_string());
        }

        if let Some(minrelease_val) = minrelease {
            sql += " AND release >= ?";
            params_vec.push(minrelease_val.to_string());
        }

        if let Some(artists_val) = artists {
            let artists_val = artists_val.into_inner();
            let artist_placeholders = artists_val.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            sql += &format!(" AND album_tracks.album_id IN ({})", artist_placeholders);
            params_vec.extend(artists_val);
        }

        if let Some(albums_val) = albums {
            let albums_val = albums_val.into_inner();
            let album_placeholders = albums_val.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            sql += &format!(" AND artist_albums.album_id IN ({})", album_placeholders);
            params_vec.extend(albums_val);
        }

        if let Some(genres_val) = genres {
            let genres_val = genres_val.into_inner();
            let genre_placeholders = genres_val.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
            sql += &format!(" AND track_genres.genre_id IN ({})", genre_placeholders);
            params_vec.extend(genres_val);
        }

        if let Some(lyrics_val) = lyrics {
            sql += " AND lyrics LIKE ?";
            params_vec.push(format!("%{}%", lyrics_val));
        }

        sql += &format!(" GROUP BY id LIMIT {}", limit.unwrap_or(50));

        let params_sql: Vec<&dyn ToSql> =
            params_vec.iter().map(|param| param as &dyn ToSql).collect();

        Ok(Json(
                conn.prepare(&sql)?.query_map(&params_sql[..], |row| {
                    let artists_str: String = row.get("artists")?;
                    let artists: Vec<String> = artists_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    let albums_str: String = row.get("albums")?;
                    let albums: Vec<String> = albums_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    let genres_str: String = row.get("genres")?;
                    let genres: Vec<String> = genres_str.split(',')
                        .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.to_string()) })
                        .collect();

                    Ok(Track {
                        id: row.get("id")?,
                        name: row.get("name")?,
                        release: row.get("release")?,
                        duration: row.get("duration")?,
                        albums,
                        artists,
                        lyrics: row.get("lyrics")?,
                        genres,
                    })
                }
                    )?
                .map(|v| v.map_err(ApiError::from))
                .collect::<Result<Vec<Track>>>()?,
               ))
    }).await
}

#[delete("/track/<id>")]
async fn track_delete(db: MyDatabase, user: User, id: String) -> Result<()> {
    if !user.permissions.contains(&Permission::TrackDelete) {
        Err(Status::Forbidden)?
    }

    db.run(move |conn| -> Result<()> {
        let tx = conn.transaction()?;

        if let Err(QueryReturnedNoRows) =
            tx.query_row("SELECT 1 FROM tracks WHERE id = ?", params![id], |_| Ok(()))
        {
            Err(Status::NotFound)?
        }

        tx.execute("DELETE FROM tracks WHERE id = ?", params![id])?;

        tx.commit()?;

        Ok(())
    })
    .await
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Track EndPoints", |rocket| async {
        rocket.mount("/", routes![track_write, track_get, track_delete])
    })
}
