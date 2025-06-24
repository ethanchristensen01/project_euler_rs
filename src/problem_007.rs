//! Find the 10001st prime number
use crate::{utils::prime::PrimeSieve, ProblemDescriptor, ProblemSolution};

pub struct Problem007;

impl ProblemDescriptor for Problem007 {
    const PROBLEM_ID: usize = 7;
    const PROBLEM_TITLE: &str = "10001st prime";
    type Solution = u64;
    const SOLUTION: Self::Solution = 104_743;
}

pub struct Solution007;

impl ProblemSolution for Solution007 {
    type Problem = Problem007;

    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
        nth_prime(10001)
    }
}

/// ONE INDEXED
#[must_use]
fn nth_prime(n: usize) -> u64 {
    let primes = PrimeSieve::first_n_primes(n).to_vec();
    *primes.last().expect("Should have generated some primes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_2nd_prime() {
        assert_eq!(nth_prime(2), 3);
    }

    #[test]
    fn problem_007_solve() {
        assert!(Solution007::test());
    }
}
