use postgres::{Connection, error::Error, types::ToSql};
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
    pub event_type: Option<i32>,
    pub after: Option<i64>,
    pub before: Option<i64>,
}

impl EventFilter {
    pub fn is_empty(&self) -> bool {
	self.origin.is_none() && self.event_type.is_none() && self.after.is_none() && self.before.is_none()
    }

    pub fn from_query(query: Option<&str>) -> Result<EventFilter, String> {
	match query {
	    None => Err("Missing query.".to_string()),
	    Some(text) => {
		let args: Vec<&str> = text.split('&').collect();
		//println!("query: {} length: {}", text, args.len());
		let mut filter = EventFilter {
		    origin: None,
		    event_type: None,
		    after: None,
		    before: None,
		};
		for arg in &args {
		    let pair: Vec<&str> = arg.split('=').collect();
		    if pair.len() != 2 {
			return Err("Invalid argument.".to_string());
		    }
		    let key = pair.get(0).unwrap();
		    let value = pair.get(1).unwrap();
		    match *key {
			"origin" => {
			    println!("Found origin = {}", value); // Handle "Field:F -> With:Strangê chars
			    filter.origin = Some(value.to_string());
			},
			"event_type" => match value.parse::<i32>() {
			    Err(_) => return Err("Invalid value.".to_string()),
			    Ok(value) => filter.event_type = Some(value),
			},
			"after" => match value.parse::<i64>() {
			    Err(_) => return Err("Invalid value.".to_string()),
			    Ok(value) => filter.after = Some(value),
			},
			"before" => match value.parse::<i64>() {
			    Err(_) => return Err("Invalid value.".to_string()),
			    Ok(value) => filter.before = Some(value),
			},
			_ => return Err("Invalid key.".to_string()),
		    }
		}
		Ok(filter)
	    },
	}
    }
}

#[test]
fn test_event_filter_errors() {
    assert_eq!(EventFilter::from_query(None).err(), Some(String::from("Missing query.")));
    assert_eq!(EventFilter::from_query(Some("")).err(), Some(String::from("Invalid argument.")));
    assert_eq!(EventFilter::from_query(Some("foo=bar")).err(), Some(String::from("Invalid key.")));
}

/*
#[test]
fn test_event_filter_values() {
    assert_eq!(EventFilter::from_query(Some("after=23")).ok().after, Some(23));
}
*/

// filter is never empty here
pub fn get_events(filter: EventFilter, conn: &Connection) -> Result<Vec<dto::Event>, Error> {
    let mut query: String = "
  SELECT e.id, \"e.from\", \"e.to\", o.origin, event_type, message
  FROM events e LEFT origins o ON e.origin_id = o.id
  WHERE ".to_string();

    //let mut params: Vec<ToSql> = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if filter.origin.is_some() {
	params.push(filter.origin.unwrap());
	query.push_str(&format!("o.origin LIKE ${}*", params.len()));
    }

    /*
    if filter.after.is_some() {
	query.push_str("\"e.to\" < $
    */

    println!("{}", query);
    Ok(vec![])
}

pub fn add_source(event_id: EventId, source: String, origin: String) {
    println!("Adding source {} ({}) to event {}", source, origin, event_id); // Again, strings, do we need separate tables?
}
