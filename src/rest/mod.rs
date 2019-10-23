extern crate hyper;
extern crate futures;

use hyper::{Body, Error, Request, Response, Method, StatusCode};
use futures::{future, Future, Stream};
use serde_json::Value;

pub mod dto;
pub mod envelope;
pub mod events;
pub mod comments;

pub type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=Error> + Send>;

pub fn router(request: Request<Body>) -> ResponseFuture {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/events") => extract_body(request, events::add_event),
        (&Method::POST, "/comments") => extract_body(request, comments::add_comment),
        (&Method::GET, path) => {
            if path.starts_with("/comments/") {
                comments::get_comments_by_event_id(path)
            } else if path.starts_with("/events/") {
                events::get_events(path) // TODO: Will get pagination and filter on origin and date
            } else {
                error_response(StatusCode::NOT_FOUND)
            }
        },
        /*
        (&Method::DELETE, path) => {
            if path.starts_with("/comments/") {
                comments::delete_comment_by_event_id(path)
            } else {
                error_response(StatusCode::NOT_FOUND)
            }
        },
         */
        _ => error_response(StatusCode::NOT_FOUND),
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

fn get_id_from_path(path: &str, sub_path: &str) -> Option<i32> {
    path.trim_start_matches(sub_path)
        .parse::<i32>()
        .ok()
        .map(|x| x as i32)
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



