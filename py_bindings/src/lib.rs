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
    // load HashSet values into PySet
    for elem in game::setup() {
        container.add(elem).unwrap();
    }
}

fn do_step(py_gil: Python, board: &PySet, ntimes: usize) {
    // empty PySet into HashSet
    let mut curr = game::Board::new();
    for _ in 0..board.len() {
        let pt: (isize, isize) = board.pop().unwrap().extract(py_gil).unwrap();
        curr.insert(pt);
    }

    let mut next = curr;
    for _ in 0..ntimes {
        next = game::next_generation(&next);
    }

    // load HashSet values back into PySet
    for elem in next {
        board.add(elem).unwrap();
    }
}
