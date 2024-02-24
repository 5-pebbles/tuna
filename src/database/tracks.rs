use rocket::serde::{self, Deserialize, Serialize};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

use core::str::FromStr;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TrackSegment {
    start: f32,
    duration: f32,
    key: u16,
    major: bool,
}

impl ToString for TrackSegment {
    fn to_string(&self) -> String {
        serde::json::to_string(&self).unwrap_or_default()
    }
}

impl FromStr for TrackSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde::json::from_str(s).map_err(|e| e.to_string())
    }
}

#[derive(FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Track {
    id: String,
    name: String,
    album: String,
    #[serde(default)]
    release: u16,
    #[serde(default)]
    duration: u32,
    #[serde(default)]
    segments: SqlVec<TrackSegment>,
    #[serde(default)]
    genres: SqlVec<String>,
    #[serde(default)]
    lyrics: String,
}
