use super::db::schemas::{Account, ActionMessage, Message};
use rocket::serde::{Deserialize, Serialize};

/* Web Schemas */

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub account: Option<Account>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub profile: Option<String>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct ActionResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub request: Option<&'a str>,
    pub action: Option<(Option<Vec<ActionMessage>>, Option<bool>)>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct QueueResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub request: Option<&'a str>,
    pub action: Option<(Option<Vec<Message>>, Option<bool>)>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContactResponse<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub success: bool,
    pub request: Option<&'a str>,
    pub response: Option<bool>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Author<'a> {
    pub name: &'a str,
    pub github: &'a str,
    pub twitter: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Index<'a> {
    pub title: &'a str,
    pub author: Author<'a>,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Register<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct VerificationCode<'a> {
    pub code: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteAccount<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct Login<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct SetProfile<'a> {
    pub token: &'a str,
    pub profile: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct GetProfile<'a> {
    pub token: &'a str,
    pub contact: &'a str,
}

/* API Schemas Functions */

#[derive(Serialize, Deserialize)]
pub struct ActionFunction<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub action: &'a str,
    pub add: Option<AddActionMessage<'a>>,
    pub get: Option<GetActionMessage<'a>>,
    pub delete: Option<DeleteActionMessage<'a>>,
}

#[derive(Serialize, Deserialize)]
pub struct QueueFunction<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub action: &'a str,
    pub add: Option<AddQueueHistory<'a>>,
    pub get: Option<GetQueueHistory<'a>>,
    pub delete: Option<DeleteQueueHistory<'a>>,
}

#[derive(Serialize, Deserialize)]
pub struct ContactFunction<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub action: &'a str,
    pub add: Option<AddContact<'a>>,
}

/* API Schemas Functions - Extra */

#[derive(Serialize, Deserialize, Clone)]
pub struct AddActionMessage<'a> {
    pub to: &'a str,
    pub action: ActionMessage,
}

#[derive(Serialize, Deserialize)]
pub struct GetActionMessage<'a> {
    pub to: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteActionMessage<'a> {
    pub to: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GetQueueHistory<'a> {
    pub token: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeleteQueueHistory<'a> {
    pub to: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddQueueHistory<'a> {
    pub to: &'a str,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddContact<'a> {
    pub action: &'a str,
    pub from: &'a str,
    pub to: &'a str,
}
