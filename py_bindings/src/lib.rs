#![feature(proc_macro)]

extern crate pyo3;
extern crate game;

use pyo3::prelude::*;

#[py::modinit(_gol)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "setup")]
    fn setup(_py: Python, container: &PySet) -> PyResult<()> {
        do_setup(container);
        return Ok(());
    }

    #[pyfn(m, "step")]
    fn step(py: Python, board: &PySet, ntimes: usize) -> PyResult<()> {
        do_step(py, board, ntimes);
        return Ok(());
    }

    return Ok(());
}

fn do_setup(container: &PySet) {
    for elem in game::setup().iter() {
        container.add(elem).unwrap();
    }
}

fn do_step(py: Python, board: &PySet, ntimes: usize) {
    let mut curr = game::Board::new();
    for _ in 0..board.len() {
        let pt: (isize, isize) = board.pop().unwrap().extract(py).unwrap();
        curr.insert(pt);
    }

    let mut next = curr;
    for _ in 0..ntimes {
        next = game::next_generation(&next);
    }

    for elem in next {
        board.add(elem).unwrap();
    }
}
