use pyo3::prelude::*;
use std::iter::Iterator;

struct FibIter {
    current: i32,
    prev: i32,
}

impl FibIter {
    fn new() -> Self {
        Self {current: 1, prev: 0}
    }
}

impl Iterator for FibIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let old = self.current;
        self.current = self.current + self.prev;
        self.prev = old;
        Some(old)
    }
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn fib(to: i32) -> PyResult<i32> {
    let mut fib = FibIter::new();
    let mut result: i32 = 0;
    for (idx, n) in fib.into_iter().enumerate() {
        println!("{n}");
        if idx == to.try_into().unwrap() {
            result = n;
            break;
        }
    }
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn fib_calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}