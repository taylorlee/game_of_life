extern crate game;

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;

use futures::{Stream, future::Future};
use hyper::{Method, StatusCode, server::{const_service, service_fn, Http, Request, Response}};

type RpcFuture = Box<Future<Item = Response, Error = hyper::Error>>;
type Data = hyper::Chunk;
type Responder = fn(Data) -> Response;

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let service = const_service(service_fn(rpc_service));
    let server = Http::new().bind(&addr, service).unwrap();
    server.run().unwrap();
}

fn rpc_service(req: Request) -> RpcFuture {
    let responder = router(&req);

    Box::new(req.body().concat2().map(responder))
}

fn router(req: &Request) -> Responder {
    match (req.method(), req.path()) {
        (&Method::Get, "/setup/") => setup,
        (&Method::Post, "/step/") => step,
        _ => not_found,
    }
}

fn not_found(_: Data) -> Response {
    Response::new().with_status(StatusCode::NotFound)
}

fn setup(_: Data) -> Response {
    let acorn = serde_json::to_string(&game::setup()).unwrap();

    Response::new().with_body(acorn)
}

fn step(data: Data) -> Response {
    let body = data.iter().cloned().collect::<Vec<u8>>();
    let (ntimes, mut board): (usize, game::Board) = serde_json::from_slice(&body).unwrap();
    for _ in 0..ntimes {
        board = game::next_generation(&board);
    }
    let resp = serde_json::to_string(&board).unwrap();

    Response::new().with_body(resp)
}
