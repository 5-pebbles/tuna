use crate::database::permissions::{permissions_from_row, Permission};
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::rusqlite::{Error, Row};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Invite {
    pub code: String,
    pub permissions: Vec<Permission>,
    pub remaining: u16,
    #[serde(skip_deserializing)]
    pub creator: String,
}

impl Invite {
    pub fn try_from_row(row: &Row) -> Result<Self, Error> {
        let permissions = permissions_from_row(row)?;
        Ok(Invite {
            code: row.get("code")?,
            permissions,
            remaining: row.get("remaining")?,
            creator: row.get("creator")?,
        })
    }
}
