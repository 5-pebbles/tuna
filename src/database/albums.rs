use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artists: Vec<String>,
    #[serde(default)]
    pub release: u16,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub count: u8,
}
