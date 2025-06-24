//! Highly Divisible Triangular Number

use crate::{
    utils::prime::{
        prime_factorize,
        PrimeSieve
    },
    ProblemDescriptor,
    ProblemSolution
};

pub struct Problem012;
pub struct Solution012;


impl ProblemDescriptor for Problem012 {
    const PROBLEM_ID: usize = 12;
    const PROBLEM_TITLE: &str = "Highly Divisible Triangular Number";
    type Solution = u64;
    const SOLUTION: Self::Solution = 76_576_500;
}

impl ProblemSolution for Solution012 {
    type Problem = Problem012;    

    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
        let primes = PrimeSieve::default();
        let result = (2_u64..)
            .scan(1, |triangle, n| {
                *triangle += n;
                Some(*triangle)
            })
            .map(|t| prime_factorize(t, &primes))
            .find_map(|pf| {
                let factor_count = pf.count_factors();
                (factor_count > 500).then(|| (pf.full_number(), factor_count))
            });

        result.expect("expected to find a result").0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_012_solve() {
        assert!(Solution012::test());
    }
}
