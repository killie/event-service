extern crate hyper;

use hyper::StatusCode;
use serde_json::Result;

use crate::db;
use crate::rest::{ResponseFuture, dto, envelope};

pub fn add_event(chunk: hyper::Chunk) -> ResponseFuture {
    // Anti-DRY ahead!

    // Read entire body from request as string
    let str_body = String::from_utf8(chunk.to_vec()).unwrap();
    // Deserialize JSON string to NewEvent
    let parse_result: Result<dto::NewEvent> = serde_json::from_str(&str_body);
    match parse_result {
        Ok(event) => {
            // Connecting to database
            match db::connect_to_db() {
                Ok(conn) => {
                    // Saving to events table
                    match db::events::create_event(event, &conn) {
                        Ok(id) => {
                            println!("id: {}", id);
                            super::id_response(id)
                        },
                        Err(e) => {
                            println!("Could not add record to database: {}", e);
                            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Invalid body: {}", str_body);
            super::empty_response(StatusCode::BAD_REQUEST)
        },
    }
}

pub fn get_events(path: &str) -> ResponseFuture {
    // Connecting to database
    match db::connect_to_db() {
        Ok(conn) => {
            // TODO: Extract valid fields from path
            let filter = db::events::EventFilter {
                origin: Some(String::from("O")),
                event_type: None,
                after: None,
                before: None,
            };
            match db::events::get_events(filter, &conn) {
                Ok(events) => {
                    let events = serde_json::to_string(&events).unwrap();
                    let envelope = envelope::success_from_str(events);
                    super::send_result(&envelope)
                },
                Err(e) => {
                    println!("Error loading comments: {}", e);
                    super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Could not connect to database.");
            super::empty_response(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
