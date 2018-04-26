use yew::services::{Task, interval::IntervalService};
use game::Board;

pub struct Model {
    pub board: Board,
    pub clock: u64,
    pub speed: u8,
    pub job: Option<Box<Task>>,
    pub running: bool,
}

pub enum Msg {
    Step,
    Incr,
    Decr,
    Start,
    Stop,
    Reset,
}

pub struct Context {
    pub interval: IntervalService<Msg>,
}
