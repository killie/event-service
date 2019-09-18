// Here we create the webserver

extern crate hyper;
extern crate futures;
//extern crate pretty_env_logger;

use hyper::{Body, Error, Request, Response, Server, Method, StatusCode};
use hyper::service::service_fn;
use futures::{future, Future};
//use event_service::models;

//mod event_service;


// https://hyper.rs/guides/server/echo/ says just a simple type alias
//type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

type OriginId = u64;
type SourceId = u64;

const ORIGIN_PATH: &str = "/origin/";
const SOURCE_PATH: &str = "/source/";

fn rest_handler(request: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {
    println!("{:?}", request);
    let response = {
        response_with_code(StatusCode::OK)
    };
    future::ok(response)
}

fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

/*
fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        },
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        },
        (method, path) if path.starts_with(ORIGIN_PATH) => {
            let origin_id = path.trim_start_matches(ORIGIN_PATH)
                .parse::<OriginId>()
                .ok()
                .map(|x| x as usize);
            
        },
        (&Method::GET, "/origins") => {
            let o1 = models::Origin { id: 1, name: "Field:F".to_string() };
            let mut origin_id = String::from("Field:F -> Well:");
            let id: u32 = 2;
            origin_id.push_str(&id.to_string());
            let o2 = models::Origin { id: 2, name: origin_id };
            /*
            let mut origins = Vec::new();
            origins.push(o1);
            origins.push(o2);
            */
            let json_string = serde_json::to_string(&o1).unwrap();
            *response.body_mut() = Body::from(json_string);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))

    // What’s with the Box? The example so far doesn’t need it, and even as we expand it, it is true
    // that you can do all these without allocating a trait object. The reason, though, is for ease.
    // We will need to return different Futures, while starting out, it’s easiest to just put all the
    // different possible return values into a boxed trait object.
}
*/

fn main() {
    //pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(move || {
            service_fn(move |req| rest_handler(req))
        })
        .map_err(|e| eprintln!("Server error: {}", e));
    
    println!("Listening on http://{}", addr);
               
    hyper::rt::run(server);
}
