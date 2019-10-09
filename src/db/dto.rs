// This is data transfer objects. They do not correspond to database model.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: i64,
    pub from: i64,
    pub to: i64, // If it is an instant then from and to is the same
    pub origin: String,
    // Vector of source names
    pub event: String,
    pub message: String,
    // Vector of comment records?
} // TODO: Related events, comments

#[derive(Serialize, Deserialize)]
pub struct Origin {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub id: u32,
    pub name: String,
    pub origin_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EventType {
    pub id: u32,
    pub name: String,
    // TODO: Add event group?
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub event_id: i64,
    pub user: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewComment {
    pub event_id: i64,
    pub user: String,
    pub text: String,
}
