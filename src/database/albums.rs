use rocket::serde::{Deserialize, Serialize};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

#[derive(FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    id: String,
    name: String,
    artists: SqlVec<String>,
    #[serde(default)]
    release: u16,
    #[serde(default)]
    genres: SqlVec<String>,
    #[serde(default)]
    count: u8,
}
