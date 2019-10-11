extern crate hyper;
extern crate futures;
#[macro_use]
extern crate diesel;
//extern crate pretty_env_logger;

use hyper::{service::service_fn, Body, Error, Request, Response, Method, StatusCode};
use futures::{future, Future, Stream};
use serde_json::{Result, Value};

mod db;
mod envelope;
mod schema;

type ResponseFuture = Box<dyn Future<Item=Response<Body>, Error=Error> + Send>;

fn router(request: Request<Body>) -> ResponseFuture {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/comments") => add_comment_handler(request),
        _ => error_response(StatusCode::NOT_FOUND),
    }
}

fn add_comment_handler(request: Request<Body>) -> ResponseFuture {
    Box::new(
        request
            .into_body()
            .concat2()
            .and_then(|whole_body| {
                // Read entire body from request as string
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                // Deserialized JSON string to NewComment
                let parse_result: Result<db::dto::NewComment> = serde_json::from_str(&str_body);
                match parse_result {
                    Ok(comment) => {
                        // Connecting to database
                        match db::connect_to_db() {
                            Some(connection) => {
                                let comment_id = db::comments::add_comment(comment, &connection);
                                println!("comment_id: {}", comment_id);
                                id_response(comment_id)
                            },
                            None => {
                                println!("Could not connect to database.");
                                error_response(StatusCode::INTERNAL_SERVER_ERROR)
                            },
                        }
                    },
                    Err(_) => {
                        println!("Invalid comment: {}", str_body);
                        error_response(StatusCode::BAD_REQUEST)
                    },
                }
            }),
    )
}

fn id_response(id: i32) -> ResponseFuture {
    let mut s = String::from(r#"{"id": "#);
    s.push_str(&format!("{}", id));
    s.push_str("}");
    match serde_json::from_str(&s) {
        Ok(json_value) => success_result(json_value),
        Err(_) => panic!("Could not parse id response."),
    }
}

fn success_result(value: Value) -> ResponseFuture {
    send_result(&envelope::success(value))
}

fn error_result(code: i32, description: String) -> ResponseFuture {
    send_result(&envelope::error(code, description))
}

fn send_result(envelope: &envelope::Envelope) -> ResponseFuture {
    match serde_json::to_string(&envelope) {
        Ok(json_str) => {
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(Body::from(json_str))
                    .unwrap()
            ))
        },
        Err(_) => panic!("Cannot serialize error message."),
    }
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
