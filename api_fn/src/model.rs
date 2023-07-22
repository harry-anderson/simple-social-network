use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStoryRequest {
    user: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentRequest {
    story_id: String,
    user: String,
    content: String,
}

/// record in dynamodb
#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    User {
        #[serde(rename = "PK")]
        pk: String,
        #[serde(rename = "SK")]
        sk: String,
        password: String,
    },
    Story {
        #[serde(rename = "PK")]
        pk: String,
        #[serde(rename = "SK")]
        sk: String,
        content: String,
    },
    Comment {
        #[serde(rename = "PK")]
        pk: String,
        #[serde(rename = "SK")]
        sk: String,
        content: String,
    },
}
