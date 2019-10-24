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
    let method = request.method();
    let path = request.uri().path();

    if let Some(_) = match_paths(path, "/events") {
        match method {
            &Method::GET => events::get_events(path),
            &Method::POST => extract_body(request, events::add_event),
            _ => empty_response(StatusCode::NOT_FOUND),
        }

    } else if let Some(ids) = match_paths(path, "/events/*") {
        match method {
            &Method::PUT => error_result(1, "Not implemented"),
            &Method::DELETE => error_result(1, "Not implemented"),
            _ => empty_response(StatusCode::NOT_FOUND),
        }

    } else if let Some(ids) = match_paths(path, "/events/*/comments") {
        let event_id = *ids.get(0).unwrap();
        match method {
            &Method::GET => comments::get_comments_by_event_id(event_id),
            &Method::POST => extract_body(request, comments::add_comment), // Duplicating event_id in path and body...
            _ => empty_response(StatusCode::NOT_FOUND),
        }

    } else if let Some(ids) = match_paths(path, "/events/*/comments/*") {
        let comment_id = *ids.get(1).unwrap();
        match method {
            &Method::PUT => error_result(1, "Not implemented"),
            &Method::DELETE => comments::delete_comment_by_id(comment_id),
            _ => empty_response(StatusCode::NOT_FOUND),
        }

    } else {
        empty_response(StatusCode::NOT_FOUND)
    }
}

// Splits a_str and b_str on / and compares parts. When b_str contains * the corresponding substring
// in a_str must contain an integer. If a_str and b_str have equal number of substrings, method returns
// Some(Vec<i32>). Otherwise it returns None to signal that there is no match.
fn match_paths(a_str: &str, b_str: &str) -> Option<Vec<i32>> {
    let a_parts: Vec<&str> = a_str.split('/').collect();
    let b_parts: Vec<&str> = b_str.split('/').collect();
    if a_parts.len() != b_parts.len() {
        return None;
    }
    let mut ids: Vec<i32> = vec![];
    for i in 0..a_parts.len() as usize {
        let a = a_parts.get(i).unwrap();
        let b = b_parts.get(i).unwrap();
        if b == &"*" {
            match a.parse::<i32>() {
                Ok(id) => ids.push(id),
                Err(_) => return None,
            }
        } else if a != b {
            return None;
        }
    }
    Some(ids)
}

#[test]
fn match_paths_test() {
    assert_eq!(match_paths("/events", "/events"), Some(vec![]));
    assert_eq!(match_paths("/events/1", "/events/*"), Some(vec![1]));
    assert_eq!(match_paths("/events/214/comments", "/events/*/comments"), Some(vec![214]));
    assert_eq!(match_paths("/events/60/comments/23", "/events/*/comments/*"), Some(vec![60, 23]));
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

fn error_result(code: i32, description: &str) -> ResponseFuture {
    send_result(&envelope::error(code, description.to_string()))
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

fn empty_response(status_code: StatusCode) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(status_code)
            .body(Body::empty())
            .unwrap()
    ))
}



