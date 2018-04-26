#![feature(proc_macro)]

extern crate pyo3;
extern crate game;

use pyo3::prelude::*;

#[py::modinit(_gol)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "setup")]
    fn setup(_py: Python, container: &PySet) -> PyResult<()> {
        for elem in game::setup().iter() {
            container.add(elem)?;
        }
        return Ok(());
    }

    #[pyfn(m, "step")]
    fn step(py: Python, board: &PySet, ntimes: usize) -> PyResult<()> {
        // convert PySet to Board
        let mut curr = game::Board::new();
        for _ in 0..board.len() {
            let pt: (isize, isize) = board.pop().unwrap().extract(py)?;
            curr.insert(pt);
        }

        // main computation
        let mut next = curr;
        for _ in 0..ntimes {
            next = game::next_generation(&next);
        }

        // convert Board back to PySet
        for elem in next {
            board.add(elem)?;
        }
        return Ok(());
    }

    return Ok(());
}
