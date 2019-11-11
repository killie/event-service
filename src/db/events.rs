use postgres::{Connection, error::Error};
use crate::rest::dto;

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

    /*
    conn.execute("INSERT INTO events (from, to, origin_id, event_type, message)
                  VALUES ($1, $2, $3, $4, $5)
                  RETURNING id",
                 &[&e.from, &e.to, &1, &1, &e.message])
     */
    Ok(1)

}

#[derive(Debug)]
pub struct EventFilter {
    pub origin: Option<String>,
    pub event_type: Option<String>,
    pub after: Option<i64>,
    pub before: Option<i64>,
}

impl EventFilter {
    pub fn from_query(query: Option<&str>) -> Result<EventFilter, String> {
	match query {
	    None => Err("Missing query.".to_string()),
	    Some(text) => {
		let args: Vec<&str> = text.split('&').collect();
		println!("query: {} length: {}", text, args.len());
		for arg in &args {
		    let pair: Vec<&str> = arg.split('=').collect();
		    if pair.len() != 2 {
			return Err("Invalid argument.".to_string());
		    }
		    let key = pair.get(0).unwrap();
		    let value = pair.get(1).unwrap();
		    match *key {
			"origin" => println!("origin = {}", value),
			"event_type" => println!("event_type = {}", value),
			"after" => println!("after = {}", value),
			"before" => println!("before = {}", value),
			_ => return Err("Invalid key".to_string()),
		    }
		}
		Ok(EventFilter {
		    origin: None,
		    event_type: Some(1.to_string()),
		    after: Some(1),
		    before: Some(1),
		})
	    },
	}
    }
}

#[test]
fn test_event_filter_errors() {
    assert_eq!(EventFilter::from_query(None).err(), Some(String::from("Missing query.")));
    assert_eq!(EventFilter::from_query(Some("")).err(), Some(String::from("Invalid argument.")));


}

pub fn get_events(filter: EventFilter, conn: &Connection) -> Result<Vec<dto::Event>, Error> {
    println!("TODO: Filter events on {:?}", filter);
    Ok(vec![])
}

pub fn add_source(event_id: EventId, source: String, origin: String) {
    println!("Adding source {} ({}) to event {}", source, origin, event_id); // Again, strings, do we need separate tables?
}
