use rocket::{
    data::ToByteUnit, fairing::AdHoc, fs::NamedFile, http::Status, tokio::fs::remove_file, Data,
};

use std::path::{Path, PathBuf};

use crate::{
    api::data::{permissions::Permission, users::User},
    database::MyDatabase,
    error::ApiError,
};

type Result<T> = std::result::Result<T, ApiError>;

/// Upload the audio file for a track.
///
/// Requires: `AudioWrite` permission.
#[utoipa::path(
    request_body(
        description = "The audio file to upload",
        content_type = "audio/mpeg",
        content = String,
    ),
    responses(
    (
        status = 200,
        description = "Success",
    ),
    (
        status = 403,
        description = "Forbidden reqiures permission `AudioWrite`",
    ),
    (
        status = 404,
        description = "The track does not exist",
    )),
    params(
        ("track", description = "The id of track for which you are uploading audio"),
    ),
    security(
        ("permissions" = ["AudioWrite"])
    ),
)]
#[put("/audio/<track>", format = "audio/mpeg", data = "<data>")]
async fn audio_upload(db: MyDatabase, user: User, track: &str, data: Data<'_>) -> Result<()> {
    if !user.permissions.contains(&Permission::AudioWrite) {
        Err(Status::Forbidden)?
    }

    // confirm that the track already exist
    let track_clone = track.to_string();

    let exists: bool = db
        .run(move |conn| {
            conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM tracks WHERE id = ?)",
                [track_clone],
                |row| row.get(0),
            )
        })
        .await?;

    if !exists {
        Err(Status::NotFound)?
    }

    // save the audio file
    data.open(20.megabytes())
        .into_file(
            Path::new("./database/audio")
                .join(track)
                .with_extension("mp3"),
        )
        .await?;
    Ok(())
}

/// Get the audio file for a track.
///
/// Requires: `AudioRead` permission.
#[utoipa::path(
    responses(
    (
        status = 200,
        description = "Success",
        content_type = "audio/mpeg",
        body = String,
    ),
    (
        status = 403,
        description = "Forbidden requires permission `AudioRead`",
    ),
    (
        status = 404,
        description = "The requested audio does not exist",
    )),
    params(
        ("track", description = "The id of the track who's audio you are downloading"),
    ),
    security(
        ("permissions" = ["AudioRead"])
    ),
)]
#[get("/audio/<track>")]
async fn audio_get(user: User, track: PathBuf) -> Result<Option<NamedFile>> {
    if !user.permissions.contains(&Permission::AudioRead) {
        Err(Status::Forbidden)?
    }

    Ok(NamedFile::open(
        Path::new("./database/audio")
            .join(track)
            .with_extension("mp3"),
    )
    .await
    .ok())
}

/// Delete the audio file for a track.
///
/// Requires: `AudioDelete` permission.
#[utoipa::path(
    responses(
    (
        status = 200,
        description = "Success",
    ),
    (
        status = 403,
        description = "Forbidden requires permission `AudioDelete`",
    ),
    (
        status = 404,
        description = "The audio file does not exist",
    )),
    params(
        ("track", description = "The id of the track who's audio you are deleting"),
    ),
    security(
        ("permissions" = ["AudioDelete"])
    ),
)]
#[delete("/audio/<track>")]
async fn audio_delete(user: User, track: PathBuf) -> Result<()> {
    if !user.permissions.contains(&Permission::AudioDelete) {
        Err(Status::Forbidden)?
    }

    let path = Path::new("./database/audio")
        .join(track)
        .with_extension("mp3");

    if !path.exists() {
        Err(Status::NotFound)?
    }

    // use tokio::fs::remove_file because it is async
    remove_file(path).await?;

    Ok(())
}

pub fn fairing() -> AdHoc {
    AdHoc::on_ignite("API Audio Endpoints", |rocket| async {
        rocket.mount("/", routes![audio_upload, audio_get, audio_delete])
    })
}
