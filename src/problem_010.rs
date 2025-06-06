//! Find the sum of all primes below two million

use crate::{Problem, utils::prime::PrimeSieve};

pub struct Problem010;

impl Problem for Problem010 {
    type Solution = u64;

    fn name() -> &'static str {
        "010 Summation of Primes"
    }

    fn solve() -> Self::Solution {
        PrimeSieve::default().take_while(|&n| n < 2_000_000).sum()
    }

    fn is_correct(solution: &Self::Solution) -> bool {
        *solution == 142_913_828_922
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_010_solve() {
        assert!(Problem010::test());
    }
}
