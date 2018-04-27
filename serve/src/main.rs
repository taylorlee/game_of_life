
extern crate game;

extern crate hyper;
extern crate futures;

extern crate serde;
extern crate serde_json;

use futures::future::Future;
use futures::Stream;

use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;


    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

         match (req.method(), req.path()) {
            (&Method::Get, "/setup") => {
                let acorn = serde_json::to_string(&game::setup()).unwrap();
                response.set_body(acorn);
            },
            (&Method::Post, "/step") => {
                println!("in");
                let mapping = req.body().concat2().map(|b| {
                    println!("in map");
                    let board: game::Board = serde_json::from_slice(b.as_ref()).unwrap();
                    let next = game::next_generation(&board);
                    println!("out map");
                    return serde_json::to_string(&next).unwrap();
                }).wait();
                let body: Box<Stream<Item=_, Error=_>> = Box::new(mapping);
                response.set_body(body);
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}
fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
