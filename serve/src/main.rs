
extern crate game;

extern crate hyper;
extern crate futures;

//extern crate serde;
//extern crate serde_json;

use futures::future::*;

use hyper::server::{Http, Request, Response, Service, const_service};
//use hyper::{Method, StatusCode};
use futures::Stream;
use hyper::*;


struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
         match (req.method(), req.path()) {
            (&Method::Get, "/setup") => {
                Box::new(ok(Response::new().with_body("ECHO")))
            },
            (&Method::Post, "/step") => {
                Box::new(req.body().concat2().map(reverse))
            },
            _ => {
                Box::new(ok(
                    Response::new().with_status(StatusCode::NotFound)
                ))
            },
         }
    }
}

fn reverse(chunk: Chunk) -> Response {
    Response::new()
        .with_body(
            chunk.iter()
            .rev()
            .cloned()
            .collect::<Vec<u8>>()
        )
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let cst = const_service(Echo);
    let server = Http::new().bind(&addr, cst).unwrap();
    server.run().unwrap();
}
