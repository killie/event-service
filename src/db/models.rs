use serde::{Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::events;
use crate::schema::comments;

#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub from: i64,
    pub to: i64, // If it is an instant then from and to is the same
    pub origin_id: i32,
    pub event_type: i32,
    pub message: String,        
}

#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct NewEvent {
    pub from: i64,
    pub to: i64,
    pub origin_id: i32,
    pub event_type: i32,
    pub message: String,        
}

#[derive(Queryable, Serialize, Debug)]
pub struct Origin {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub origin_id: i32,
}

#[derive(Queryable, Serialize, Debug)]
pub struct EventSource {
    pub id: i32,
    pub event_id: i32,
    pub source_id: i32,
}

#[derive(Queryable, Serialize, Debug)]
pub struct EventType {
    pub id: i32,
    pub name: String,
    pub description: String,
    // TODO: Do we also need event group?
}

#[derive(Queryable, Serialize, Debug)]
pub struct Comment {
    pub id: i32,
    pub event_id: i32,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    pub event_id: i32,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}


