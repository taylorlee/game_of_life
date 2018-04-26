#![recursion_limit = "256"] // needed for html! macro expansion

mod model;
mod actions;
mod view;

#[macro_use]
extern crate yew;
extern crate game;

use yew::{initialize, run_loop, html::App, services::interval::IntervalService};

fn main() {
    initialize();
    let mut app = App::new();
    let context = model::Context {
        interval: IntervalService::new(app.sender()),
    };
    let model = model::Model {
        board: game::setup(),
        speed: 5,
        clock: 0,
        job: None,
        running: false,
    };
    app.mount(context, model, actions::update, view::render);
    run_loop();
}
