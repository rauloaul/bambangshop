use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use create::model::notification::Notification;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct Subscriber {
    pub url: String,
    pub name: String,
}