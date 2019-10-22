use postgres::{Connection, error::Error};
use crate::dto;

type EventId = i32;

pub fn create_event(e: dto::NewEvent, conn: &Connection) -> Result<EventId, Error> {
    // NewEvent has origin and eventType as strings. These must be added to separate tables,
    // and corresponding id must be used as origin_id and event_type in events table.
    /*
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
     */

    conn.execute("INSERT INTO events (from, to, origin_id, event_type, message)
                  VALUES ($1, $2, $3, $4, $5)",
                 &[&e.from, &e.to, &1, &1, &e.message]);
    // Returning id
    Ok(0)
}

#[derive(Debug)]
pub struct EventFilter {
    pub origin: Option<String>,
    pub event_type: Option<String>,
    pub after: Option<i64>,
    pub before: Option<i64>,
}

pub fn get_events(filter: EventFilter, conn: &Connection) -> Result<Vec<dto::Event>, Error> {
    println!("TODO: Filter events on {:?}", filter);
    Ok(vec![])
}

pub fn add_source(event_id: EventId, source: String, origin: String) {
    println!("Adding source {} ({}) to event {}", source, origin, event_id); // Again, strings, do we need separate tables?
}
