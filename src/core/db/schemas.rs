use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    #[serde(rename = "_id")]
    pub id: String,
    pub secret: String,
}

/* USER SCHEMAS */

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    pub remaining: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalAccount {
    #[serde(rename = "_id")]
    pub id: String,
    pub profile: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub verification: Verification,
    pub contacts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: String,
    pub profile: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub contacts: Vec<String>,
}

/* ACTIONS SCHEMAS */

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionBase {
    #[serde(rename = "_id")]
    pub id: String,
    pub actions: Vec<ActionMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionMessage {
    pub title: String,
    pub from: String,
    pub message_id: String,
    pub action: String,
    pub date: String,
}

/* MESSAGES SCHEMAS */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub contain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageBase {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub messages: Vec<Message>,
    pub date: String,
}
