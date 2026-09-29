#![feature(iter_intersperse)]

pub type Column = Vec<usize>;
pub type Matrix = Vec<Column>;

#[must_use]
pub fn diagonal(n: usize) -> Column {
    if n == 0 {
        vec![0]
    } else {
        (1..=n).rev().collect()
    }
}

pub fn limit(n: usize) -> Matrix {
    (0..n).map(diagonal).collect()
}

#[must_use]
pub fn copy_n_times(v: &Matrix, i: usize) -> Matrix {
    v.iter().cycle().take(v.len() * i).cloned().collect()
}

pub mod expansion;
pub use expansion::{expand, expand_with_rule};

pub mod codec;

#[cfg(test)]
mod tests;
