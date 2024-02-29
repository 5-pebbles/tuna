use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Vec<String>,
    pub bio: String,
}
