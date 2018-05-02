
extern crate game;

extern crate hyper;
extern crate futures;

extern crate serde;
extern crate serde_json;

//use std::ascii::AsciiExt;
use futures::future::Future;
use futures::Stream;

use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use hyper::*;

//use futures::future::FutureResult;

struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    //type Future = Future<Item=Self::Response, Error=Self::Error>;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
         match (req.method(), req.path()) {
            (&Method::Get, "/setup") => {
                Box::new(futures::future::ok(Response::new().with_body("ECHO")))
            },
            (&Method::Post, "/step") => {
                Box::new(req.body().concat2().map(reverse))
            },
            _ => {
                Box::new(futures::future::ok(Response::new().with_status(StatusCode::NotFound)))
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
fn to_upper(chunk: Chunk) -> Chunk {
    let uppered: Vec<u8> = chunk.iter()
        .map(|b| b.to_ascii_uppercase())
        .collect();
    Chunk::from(uppered)
}
//let board: game::Board = serde_json::from_slice(req.body().as_ref().into()).unwrap();
//let next = game::next_generation(&board);
//let resp = serde_json::to_string(&next).unwrap();
//
//                //let acorn = serde_json::to_string(&game::setup()).unwrap();
                //let transform = req.body().map(to_upper);
                //let body: Box<Stream<Item=_,Error=_>> = Box::new(transform);
                //response.set_body(body);


fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || {return Ok((Echo));}).unwrap();
    server.run().unwrap();
}
