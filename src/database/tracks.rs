use rocket::serde::{self, Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Track {
    pub id: String,
    pub name: String,
    pub album: String,
    #[serde(default)]
    pub release: u16,
    #[serde(default)]
    pub duration: u32,
    #[serde(default)]
    pub segments: SqlVec<TrackSegment>,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub lyrics: String,
}
