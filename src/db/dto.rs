// This is data transfer objects. They do not correspond to database model.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: i32,
    pub from: i64,
    pub to: i64, // If it is an instant then from and to is the same
    pub origin: String,
    // Vector of source names
    pub eventType: String,
    pub message: String,
    // Vector of comment records?
} // TODO: Related events, comments

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEvent {
    pub from: i64,
    pub to: i64,
    pub origin: String,
    pub eventType: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Origin {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub origin_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct EventType {
    pub id: i32,
    pub name: String,
    // TODO: Add event group?
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub event_id: i32,
    pub user: String,
    pub text: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewComment {
    pub event_id: i32,
    pub user: String,
    pub text: String,
    pub timestamp: i64,
}
