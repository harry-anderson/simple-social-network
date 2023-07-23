use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserInput {
    pub user_id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUserInput {
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStoryInput {
    pub user_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteStoryInput {
    pub user_id: String,
    pub story_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentInput {
    pub story_id: String,
    pub user_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCommentInput {
    pub comment_id: String,
    pub story_id: String,
}

/// record in dynamodb
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
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
        story_text: String,
    },
    Comment {
        #[serde(rename = "PK")]
        pk: String,
        #[serde(rename = "SK")]
        sk: String,
        user_id: String,
        comment_text: String,
    },
}
