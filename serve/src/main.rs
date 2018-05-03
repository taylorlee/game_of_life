
extern crate game;

extern crate hyper;
extern crate futures;

extern crate serde;
extern crate serde_json;

use futures::future::*;
use futures::Stream;

use hyper::server::{Http, Request, Response, Service, const_service};
use hyper::*;

struct RPC;

impl Service for RPC {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
         match (req.method(), req.path()) {
            (&Method::Get, "/setup/") => {
                let acorn = serde_json::to_string(&game::setup()).unwrap();

                Box::new(ok(Response::new().with_body(acorn)))
            },
            (&Method::Post, "/step/") => {
                Box::new(req.body().concat2().map(|chunk| {
                    let body = chunk.iter()
                        .cloned()
                        .collect::<Vec<u8>>();
                    let board: game::Board = serde_json::from_slice(&body).unwrap();
                    let next = game::next_generation(&board);
                    let resp = serde_json::to_string(&next).unwrap();

                    Response::new().with_body(resp)
                }))
            },
            _ => {
                Box::new(ok(Response::new().with_status(StatusCode::NotFound)))
            },
         }
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, const_service(RPC)).unwrap();
    server.run().unwrap();
}
