extern crate hyper;
extern crate futures;
//extern crate pretty_env_logger;

use hyper::{service::service_fn, Body, Error, Request, Response, Method, StatusCode};
use futures::{future, Future, Stream};
use serde_json::{Result, Value};

mod db;
mod dto;
mod envelope;
    
type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=Error> + Send>;

fn router(request: Request<Body>) -> ResponseFuture {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/events") => extract_body(request, add_event),
        (&Method::POST, "/comments") => extract_body(request, add_comment),
        (&Method::GET, path) => {
            if path.starts_with("/comments/") {
                get_comments_by_event_id(path)
            } else if path.starts_with("/events/") {
                get_events(path) // TODO: Will get pagination and filter on origin and date
            } else {
                error_response(StatusCode::NOT_FOUND)
            }
        },
        _ => error_response(StatusCode::NOT_FOUND),
    }
}

fn add_event(chunk: hyper::Chunk) -> ResponseFuture {
    // Anti-DRY ahead!

    // Read entire body from request as string
    let str_body = String::from_utf8(chunk.to_vec()).unwrap();
    // Deserialize JSON string to NewEvent
    let parse_result: Result<dto::NewEvent> = serde_json::from_str(&str_body);
    match parse_result {
        Ok(event) => {
            // Connecting to database
            match db::connect_to_db() {
                Ok(connection) => {
                    // Saving to events table
                    match db::events::create_event(event, &connection) {
                        Ok(id) => {
                            println!("id: {}", id);
                            id_response(id)
                        },
                        Err(error) => {
                            println!("Could not add record to database: {}", error);
                            error_response(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    error_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Invalid body: {}", str_body);
            error_response(StatusCode::BAD_REQUEST)
        },
    }
}

fn get_events(path: &str) -> ResponseFuture {
    // Connecting to database
    match db::connect_to_db() {
        Ok(connection) => {
            // TODO: Extract valid fields from path
            let filter = db::events::EventFilter {
                origin: Some(String::from("O")),
                event_type: None,
                after: None,
                before: None,
            };
            match db::events::get_events(filter, &connection) {
                Ok(events) => {
                    let events = serde_json::to_string(&events).unwrap();
                    let envelope = envelope::success_from_str(events);
                    send_result(&envelope)
                },
                Err(error) => {
                    println!("Error loading comments");
                    error_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Could not connect to database.");
            error_response(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

fn add_comment(chunk: hyper::Chunk) -> ResponseFuture {
    // Read entire body from request as string
    let str_body = String::from_utf8(chunk.to_vec()).unwrap();
    // Deserialize JSON string to NewComment
    let parse_result: Result<dto::NewComment> = serde_json::from_str(&str_body);
    match parse_result {
        Ok(comment) => {
            // Connecting to database
            match db::connect_to_db() {
                Ok(connection) => {
                    // Saving to comments table
                    match db::comments::add_comment(comment, &connection) {
                        Ok(id) => {
                            println!("id: {}", id);
                            id_response(id)
                        },
                        Err(error) => {
                            println!("Could not add record to database: {}", error);
                            error_response(StatusCode::INTERNAL_SERVER_ERROR)
                        }}
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    error_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        Err(_) => {
            println!("Invalid body: {}", str_body);
            error_response(StatusCode::BAD_REQUEST)
        },
    }
}

fn get_comments_by_event_id(path: &str) -> ResponseFuture {
    // Get event_id from request path
    let parse_result = path.trim_start_matches("/comments/")
        .parse::<i32>()
        .ok()
        .map(|x| x as i32);

    match parse_result {
        Some(event_id) => {
            // Connecting to database
            match db::connect_to_db() {
                Ok(connection) => {
                    // Loading comments on said event_id
                    match db::comments::get_comments(event_id, &connection) {
                        Ok(comments) => {
                            let comments = serde_json::to_string(&comments).unwrap();
                            let envelope = envelope::success_from_str(comments);
                            send_result(&envelope)
                        },
                        Err(error) => {
                            println!("Error loading comments");
                            error_response(StatusCode::INTERNAL_SERVER_ERROR)
                        },
                    }
                },
                Err(_) => {
                    println!("Could not connect to database.");
                    error_response(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }
        },
        None => {
            error_response(StatusCode::BAD_REQUEST)
        },
    }
}

fn extract_body(request: Request<Body>, body_handler: fn(chunk: hyper::Chunk) -> ResponseFuture) -> ResponseFuture {
    Box::new(
        request
            .into_body()
            .concat2()
            .and_then(body_handler)
    )
}

fn id_response(id: i32) -> ResponseFuture {
    let mut s = String::from(r#"{"id": "#);
    s.push_str(&format!("{}", id));
    s.push_str("}");
    let json_value = serde_json::from_str(&s).unwrap();
    success_result(json_value)
}

fn success_result(value: Value) -> ResponseFuture {
    send_result(&envelope::success(value))
}

fn error_result(code: i32, description: String) -> ResponseFuture {
    send_result(&envelope::error(code, description))
}

fn send_result(envelope: &envelope::Envelope) -> ResponseFuture {
    let json_str = serde_json::to_string(&envelope).unwrap();
    Box::new(future::ok(
        Response::builder()
            .header("Content-Type", "application/json")
            .status(StatusCode::OK)
            .body(Body::from(json_str))
            .unwrap()
    ))
}

fn error_response(status_code: StatusCode) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(status_code)
            .body(Body::empty())
            .unwrap()
    ))
}
 
fn main() {
    //pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr)
        .serve(move || {
            service_fn(move |request| router(request))
        })
        .map_err(|e| eprintln!("Server error: {}", e));
    
    println!("Listening on http://{}", addr);
               
    hyper::rt::run(server);
}
