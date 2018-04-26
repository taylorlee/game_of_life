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

    #[pyfn(m, "step", args="*", kwargs="**")]
    fn step(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<()> {
        // unpack args and kwargs
        let board: &PySet = args.get_item(0).extract()?;
        let ntimes = match kwargs {
            Some(kwargs) => kwargs.get_item("ntimes").unwrap().extract()?,
            None => 1,
        };

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
