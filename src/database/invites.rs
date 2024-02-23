use crate::database::permissions::Permission;
use rocket::serde::{Deserialize, Serialize};
use rusqlite_from_row::FromRow;
use sqlvec::SqlVec;

#[derive(Clone, FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Invite {
    pub code: String,
    pub permissions: SqlVec<Permission>,
    pub remaining: u16,
    #[serde(skip_deserializing)]
    pub creator: String,
}
