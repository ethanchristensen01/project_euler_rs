//! Find the only pythagorean triple which sums to 1000, and return the product of each number

use crate::{ProblemDescriptor, ProblemSolution};

pub struct Problem009;
pub struct Solution009;

impl ProblemDescriptor for Problem009 {
    const PROBLEM_ID: usize = 9;
    const PROBLEM_TITLE: &str = "Special Pythagorean Triplet";
    type Solution = u64;
    const SOLUTION: Self::Solution = 31_875_000;
}
impl ProblemSolution for Solution009 {
    type Problem = Problem009;

    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
        find_pythagorean_triple_1000().expect("problem 9 solution should not be none")
    }
}

// TODO: Could generalize this problem a little more, maybe?
//   Would require const prime_factorization, which exists here https://crates.io/crates/const-primes

pub fn get_possible_products() -> impl Iterator<Item = u64> {
    (0..=5)
        .flat_map(|a| (0..=6).map(move |b| (a, b)))
        .map(|(a, b)| 2_u64.pow(a) * 5_u64.pow(b))
        .filter(|&p| p < 500)
}

#[must_use]
pub fn find_pythagorean_triple_1000() -> Option<u64> {
    get_possible_products().find_map(|a| {
        let b = 1000 - 500_000 / (1000 - a);
        let c = 1000_u64.checked_sub(a + b)?;
        (a * a + b * b == c * c).then_some(a * b * c)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_009_solve() {
        assert!(Solution009::test());
    }
}
