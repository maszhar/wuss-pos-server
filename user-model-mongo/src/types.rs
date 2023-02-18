use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub struct User {
    pub id: String,
    pub username: String,
    pub fullname: String,
    pub password: String
}