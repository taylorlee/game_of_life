
extern crate game;

extern crate hyper;
extern crate futures;

extern crate serde;
extern crate serde_json;

use std::ascii::AsciiExt;
use futures::future::Future;
use futures::Stream;

use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use hyper::{Body, Chunk};

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
         match (req.method(), req.path()) {
            (&Method::Get, "/setup") => {
                //let acorn = serde_json::to_string(&game::setup()).unwrap();
                let body: Box<Stream<Item=_,Error=_>> = Box::new(Body::from("ECHO"));
                response.set_body(body);
            },
            (&Method::Post, "/step") => {
                let transform = req.body().map(to_upper);
                let body: Box<Stream<Item=_,Error=_>> = Box::new(transform);
                response.set_body(body);
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}
fn to_upper(chunk: Chunk) -> Chunk {
    Chunk::from(
        chunk.iter()
        .map(|b| b.to_ascii_uppercase())
        .collect()
    )
}
//let board: game::Board = serde_json::from_slice(req.body().as_ref().into()).unwrap();
//let next = game::next_generation(&board);
//let resp = serde_json::to_string(&next).unwrap();

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
