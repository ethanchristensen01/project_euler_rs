//! Longest Collatz Sequence

use crate::{
    utils::collatz::{
        Collatz
    },
    ProblemDescriptor,
    ProblemSolution
};

pub struct Problem013;
pub struct Solution013;


impl ProblemDescriptor for Problem013 {
    const PROBLEM_ID: usize = 13;
    const PROBLEM_TITLE: &str = "Longest Collatz Sequence";
    type Solution = usize;
    const SOLUTION: Self::Solution = 837_799;
}

impl ProblemSolution for Solution013 {
    type Problem = Problem013;

    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
        let mut collatz = Collatz::with_capacity(1_000_000);
        collatz.run();
        collatz.best.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_013_solve() {
        assert!(Solution013::test());
    }
}
