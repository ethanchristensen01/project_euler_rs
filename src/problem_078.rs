//! Coin Partitions

use std::{cmp::min};

use crate::{
    ProblemDescriptor,
    ProblemSolution
};

pub struct Problem078;
pub struct Solution078;

// p(n) = P(n, n)
// P(n, g) = P(n - g, Min(g, n - g))
// P(0, g) = 1
// P(g, 0) = 0
pub struct DynamicPartitions {
    data: Vec<Vec<u32>>
}

pub enum DynamicPartitionsError {
    AdditionOverflow {
        a: u32, b: u32, col: usize, row: usize
    },
    OutOfBounds {
        col: usize, row: usize
    }
}

impl DynamicPartitions {
    #[must_use]
    pub fn new () -> Self {
        Self { data: vec![vec![1]] }
    }

    #[must_use]
    pub fn access (&self, amount: usize, g: usize) -> Option<u32> {
        let row = min(g, amount);
        self.data.get(amount).and_then(|v| v.get(row)).copied()
    }

    /// # Errors
    /// - If integer overflow is reached
    /// - If out-of-bounds access is attempted during step
    pub fn step (&mut self) -> Result<u32, DynamicPartitionsError> {
        let amount = self.data.len();
        let mut results = vec![0; amount + 1];
        let mut sum: u32 = 0;
        for n in (0..amount).rev() {
            let g = amount - n;
            let dep = self.access(n, g).ok_or(DynamicPartitionsError::OutOfBounds { col: n, row: g })?;
            sum = (sum + dep) % 1_000_000;
            results[g] = sum;
        }
        self.data.push(results);
        Ok(sum)
    }
}

impl Default for DynamicPartitions {
    fn default() -> Self {
        Self::new()
    }
}

impl ProblemDescriptor for Problem078 {
    const PROBLEM_ID: usize = 78;
    const PROBLEM_TITLE: &str = "Coin Partitions";
    type Solution = u32;
    const SOLUTION: Self::Solution = 0;
}

impl ProblemSolution for Solution078 {
    type Problem = Problem078;

    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_078_solve() {
        assert!(Solution078::test());
    }

    #[test]
    fn checking_dev() {
        let mut d = DynamicPartitions::new();
        let res = loop {
            let step = d.step();
            match &step {
                Ok(r) if r % 1_000_000 == 0 => break step,
                Err(_) => break step,
                Ok(_) => {}
            }
        };
        match res {
            Ok(n) => println!("Found: {n} at {}", d.data.len()),
            Err(DynamicPartitionsError::AdditionOverflow{ a, b, row, col }) => println!("Tried to add {a} + {b} at {row}, {col}"),
            Err(DynamicPartitionsError::OutOfBounds { col, row }) => println!("Tried to access {row}, {col}")
        }
        let lasts: Vec<_> = d.data.iter().take(53).map(|v| v.last()).collect();
        dbg!(lasts);
    }
}
