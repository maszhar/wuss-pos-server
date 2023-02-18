use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub fullname: String,
    pub password: String
}