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
        content = Data,
        description = "The audio file to upload",
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
async fn upload_audio(db: MyDatabase, user: User, track: &str, data: Data<'_>) -> Result<()> {
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
#[get("/audio/<track>")]
async fn get_audio(user: User, track: PathBuf) -> Result<Option<NamedFile>> {
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
#[delete("/audio/<track>")]
async fn delete_audio(user: User, track: PathBuf) -> Result<()> {
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
        rocket.mount("/", routes![upload_audio, get_audio, delete_audio])
    })
}
