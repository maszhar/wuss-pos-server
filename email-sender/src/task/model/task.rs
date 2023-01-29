use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub _id: ObjectId,
    pub scope: String,
    pub command: String,
    pub arg0: Option<String>,
    pub arg1: Option<String>,
    pub handler: Option<String>,
    pub failed_count: u32,
    pub created_at: DateTime,
    pub handled_at: Option<DateTime>,
    pub done_at: Option<DateTime>,
}