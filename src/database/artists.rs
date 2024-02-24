use rocket::serde::{Deserialize, Serialize};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

#[derive(FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Artist {
    id: String,
    name: String,
    #[serde(default)]
    genres: SqlVec<String>,
    #[serde(default)]
    bio: String,
}
