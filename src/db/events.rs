use diesel::pg::{PgConnection};
use diesel::result::{Error as DieselError};
use crate::diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

pub use super::dto; // TODO: Remove from db mod into root
use super::models;
use crate::schema::events;

type EventId = i32;

pub fn create_event(event: dto::NewEvent, connection: &PgConnection) -> Result<EventId, DieselError> {
    // NewEvent has origin and eventType as strings. These must be added to separate tables,
    // and corresponding id must be used as origin_id and event_type in events table.
    
    let new_event = models::NewEvent {
        from: event.from,
        to: event.to,
        origin_id: 0, //event.origin,
        event_type: 0, //event.eventType,
        message: event.message,
    };

    diesel::insert_into(events::table)
        .values(&new_event)
        .returning(events::id)
        .get_result(connection)
}

#[derive(Debug)]
pub struct EventFilter {
    pub origin: Option<String>,
    pub event_type: Option<String>,
    pub after: Option<i64>,
    pub before: Option<i64>,
}

pub fn get_events(filter: EventFilter, connection: &PgConnection) -> Result<Vec<dto::Event>, DieselError> {
    println!("TODO: Filter events on {:?}", filter);
    Ok(vec![])
}

pub fn add_source(event_id: EventId, source: String) {
    println!("Adding source {} to event {}", source, event_id);
}
