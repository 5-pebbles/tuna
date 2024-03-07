use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Track {
    pub id: String,
    pub name: String,
    pub release: u16,
    #[serde(skip_deserializing)]
    pub duration: u32,
    #[serde(default)]
    pub albums: Vec<String>,
    #[serde(skip_deserializing)]
    pub artists: Vec<String>,
    #[serde(default)]
    pub lyrics: String,
    #[serde(default)]
    pub genres: Vec<String>,
}
