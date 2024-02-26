use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Artist {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub genres: Vec<String>,
    #[serde(default)]
    pub bio: String,
}
