use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePost {
    pub id: ObjectId,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: ObjectId,
    pub title: String,
    pub content: String,
}
