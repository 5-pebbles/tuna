use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    pub id: String,
    pub name: String,
    pub release: u16,
    #[serde(default)]
    pub artists: Vec<String>,
    #[serde(default)]
    pub tracks: Vec<String>,
    #[serde(default)]
    pub genres: Vec<String>,
}
