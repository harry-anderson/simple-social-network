use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    user: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    story_id: String,
    user: String,
    content: String,
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

impl From<User> for Entity {
    fn from(req: User) -> Self {
        Entity::User {
            pk: format!("user#{}", req.username),
            sk: format!("user#{}", req.username),
            password: req.password,
        }
    }
}

impl From<Story> for Entity {
    fn from(req: Story) -> Self {
        Entity::Story {
            pk: format!("user#{}", req.user),
            sk: format!("story#{}", uuid::Uuid::new_v4()),
            story_text: req.content,
        }
    }
}

impl From<Comment> for Entity {
    fn from(req: Comment) -> Self {
        Entity::Comment {
            pk: format!("comment#{}", uuid::Uuid::new_v4()),
            sk: format!("story#{}", req.story_id),
            comment_text: req.content,
        }
    }
}
