use std::time::Duration;

use model::{Context, Model, Msg};
use game;

pub fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Start => {
            do_start(context, model);
        }
        Msg::Stop => {
            do_stop(model);
        }
        Msg::Reset => {
            model.board = game::setup();
            model.clock = 0;
        }
        Msg::Step => {
            model.clock += 1;
            model.board = game::next_generation(&model.board);
        }
        Msg::Incr => {
            if model.speed < 10 {
                model.speed += 1;
            }
            if model.running {
                restart(context, model);
            }
        }
        Msg::Decr => {
            if model.speed > 0 {
                model.speed -= 1;
            }
            if model.running {
                restart(context, model);
            }
        }
    };
}

fn cycle_time(speed: u8) -> Duration {
    Duration::from_millis(1000 - (100 * (speed as u64)))
}

fn do_start(context: &mut Context, model: &mut Model) {
    let timeout = cycle_time(model.speed);
    let handle = context.interval.spawn(timeout, || Msg::Step);
    model.job = Some(Box::new(handle));
    model.running = true;
}

fn do_stop(model: &mut Model) {
    if let Some(mut task) = model.job.take() {
        task.cancel();
    }
    model.job = None;
    model.running = false;
}

fn restart(context: &mut Context, model: &mut Model) {
    do_stop(model);
    do_start(context, model);
}
