
extern crate hyper;
extern crate futures;
#[macro_use]
extern crate diesel;
//extern crate pretty_env_logger;
extern crate url;

//use std::collections::HashMap;
//use std::io;
use hyper::{service::service_fn, Body, Error, Request, Response, Method, StatusCode, Chunk};
use futures::{future, Future, future::FutureResult, Stream};
use serde_json::Result;

mod db;


type EventId = i64;

type ResponseFuture = Box<Future<Item=Response<Body>, Error=Error> + Send>;

fn router(request: Request<Body>) -> ResponseFuture {
    match (request.method(), request.uri().path()) {
        (&Method::POST, "/comments") => add_comment_handler(request),
        _ => four_oh_four(),
    }
}

fn four_oh_four() -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap(),
    ))
}

fn add_comment_handler(request: Request<Body>) -> ResponseFuture {
    Box::new(
        request
            .into_body()
            .concat2()
            .and_then(|whole_body| {
                let str_body = String::from_utf8(whole_body.to_vec()).unwrap();
                let parse_result: Result<db::dto::NewComment> = serde_json::from_str(&str_body);
                match parse_result {
                    Ok(comment) => {
                        println!("comment: {:?}", comment);
                        redirect_home(&comment.text)
                    },
                    Err(_) => {
                        println!("Invalid comment: {}", str_body);
                        redirect_home(&"Error".to_string())
                    },
                }
            }),
    )
}

fn redirect_home(text: &String) -> ResponseFuture {
    Box::new(future::ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(text.clone()))
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
