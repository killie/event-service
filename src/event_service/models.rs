
#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: u64,
    pub from: i64,
    pub to: i64, // If it is an instant then from and to is the same
    pub origin_id: u32,
    pub event_type: u32,
    pub message: String,        
}

#[derive(Queryable, Serialize, Debug)]
pub struct Origin {
    pub id: u32,
    pub name: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Source {
    pub id: u32,
    pub name: String,
    pub origin_id: u32,
}

#[derive(Queryable, Serialize, Debug)]
pub struct EventSource {
    pub id: u64,
    pub event_id: u64,
    pub source_id: u64,
}

#[derive(Queryable, Serialize, Debug)]
pub struct EventType {
    pub id: u32,
    pub name: String,
    pub description: String,
    // TODO: Do we also need event group?
}

#[derive(Queryable, Serialize, Debug)]
pub struct Comment {
    pub id: i64,
    pub event_id: i64,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    pub event_id: i64,
    pub username: String,
    pub message: String,
} // TODO: Timestamp should come from timer engine (client) and not system


