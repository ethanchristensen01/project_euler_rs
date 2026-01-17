//! Coin Partitions

use std::cmp::min;

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
struct DynamicPartitions {
    data: Vec<Vec<usize>>
}

impl DynamicPartitions {
    fn new () -> Self {
        Self { data: vec![vec![1]] }
    }

    fn access (&self, amount: usize, g: usize) -> Option<usize> {
        let row = min(g, amount);
        self.data.get(amount).and_then(|v| v.get(g)).copied()
    }
    
    fn step (&mut self) {
        let amount = self.data.len();
        let mut sum = 0;
        for n in 0..amount {
            let g = amount - n;
            sum += self.access(n, g).expect("Should not access outside of what is accessible");
        }
        self.data.insert
    }
}

impl ProblemDescriptor for Problem078 {
    const PROBLEM_ID: usize = 78;
    const PROBLEM_TITLE: &str = "Coin Partitions";
    type Solution = usize;
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
}
