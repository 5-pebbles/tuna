use rocket::serde::{Deserialize, Serialize};
use rusqlite_from_row::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Genre {
    pub id: u32,
    pub name: String,
}
