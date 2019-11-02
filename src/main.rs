extern crate hyper;
extern crate futures;

use hyper::service::service_fn;
use futures::Future;

mod db;
mod rest;    

fn main() {
    // Update database
    match db::run_updates() {
        Ok(_) => (),
        Err(e) => {
            println!("{:?}", e);
            panic!("Preparing database.")
        },
    }

    //pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = hyper::Server::bind(&addr)
        .serve(move || {
            service_fn(move |request| rest::router(request))
        })
        .map_err(|e| eprintln!("Server error: {}", e));
    
    println!("Listening on http://{}", addr);
               
    hyper::rt::run(server);
}
