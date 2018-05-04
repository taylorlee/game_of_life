extern crate game;

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;

use futures::future::Future;
use futures::Stream;

use hyper::{Chunk, Error, Method, StatusCode};
use hyper::server::{const_service, service_fn, Http, Request, Response};

type RpcFuture = Box<Future<Item = Response, Error = Error>>;
type Data = Chunk;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let service = const_service(service_fn(router));
    let server = Http::new().bind(&addr, service).unwrap();
    server.run().unwrap();
}

fn router(req: Request) -> RpcFuture {
    return match (req.method(), req.path()) {
        (&Method::Get, "/setup/") => handler(req, setup),
        (&Method::Post, "/step/") => handler(req, step),
        (&Method::Post, "/step/many/") => handler(req, step_many),
        _ => handler(req, not_found),
    };
}

fn handler(req: Request, func: fn(Data) -> Response) -> RpcFuture {
    return Box::new(req.body().concat2().map(func));
}

fn not_found(_: Data) -> Response {
    return Response::new().with_status(StatusCode::NotFound);
}

fn setup(_: Data) -> Response {
    let acorn = serde_json::to_string(&game::setup()).unwrap();
    return Response::new().with_body(acorn);
}

fn step(data: Data) -> Response {
    let body = data.iter().cloned().collect::<Vec<u8>>();
    let board: game::Board = serde_json::from_slice(&body).unwrap();
    let next = game::next_generation(&board);
    let resp = serde_json::to_string(&next).unwrap();
    return Response::new().with_body(resp);
}

fn step_many(data: Data) -> Response {
    let body = data.iter().cloned().collect::<Vec<u8>>();
    let (ntimes, mut board): (usize, game::Board) = serde_json::from_slice(&body).unwrap();
    for _ in 0..ntimes {
        board = game::next_generation(&board);
    }
    let resp = serde_json::to_string(&board).unwrap();
    return Response::new().with_body(resp);
}
