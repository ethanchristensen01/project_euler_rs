//! Find the sum of all primes below two million

use crate::{ProblemDescriptor, ProblemSolution, utils::prime::PrimeSieve};

pub struct Problem010;
pub struct Solution010;

impl ProblemDescriptor for Problem010 {
    const PROBLEM_ID: usize = 10;
    const PROBLEM_TITLE: &str = "Summation of Primes";
    type Solution = u64;
    const SOLUTION: Self::Solution = 142_913_828_922;
}

impl ProblemSolution for Solution010 {
    type Problem = Problem010;
    
    fn solve() -> u64 {
        PrimeSieve::default().iter().take_while(|&n| n < 2_000_000).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_010_solve() {
        assert!(Solution010::test());
    }
}
