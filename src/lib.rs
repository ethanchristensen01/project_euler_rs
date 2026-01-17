pub mod utils;

pub mod problem_004;
pub mod problem_007;
pub mod problem_008;
pub mod problem_009;
pub mod problem_010;
pub mod problem_011;
pub mod problem_012;
pub mod problem_013;
pub mod problem_078;

use inflector::Inflector;

pub trait ProblemDescriptor {
    const PROBLEM_ID: usize;
    const PROBLEM_TITLE: &str;
    type Solution: PartialEq + std::fmt::Debug;
    const SOLUTION: Self::Solution;

    fn is_correct (solution: &Self::Solution) -> bool {
        Self::SOLUTION.eq(solution)
    }

    #[must_use]
    fn get_descriptor_str() -> String {
        format!("problem_{:03}_{}", Self::PROBLEM_ID, Self::PROBLEM_TITLE.to_snake_case())
    }
}

pub trait ProblemSolution {
    type Problem: ProblemDescriptor;
    const SOLUTION_ID: char = 'A';
    fn solve() -> <Self::Problem as ProblemDescriptor>::Solution;

    #[cfg(debug_assertions)]
    fn dbg_solve() {
        dbg!(Self::solve());
    }

    #[must_use]
    fn get_descriptor_str() -> String {
        format!(
            "{}_solution_{}",
            <Self::Problem as ProblemDescriptor>::get_descriptor_str(),
            Self::SOLUTION_ID.to_uppercase()
        )
    }

    #[must_use]
    fn test() -> bool {
        Self::Problem::is_correct(&Self::solve())
    }
}

#[cfg(test)]
mod tests {
    use super::{ProblemSolution, ProblemDescriptor};

    struct Problem00X;
    struct Solution00X;

    impl ProblemDescriptor for Problem00X {
        const PROBLEM_ID: usize = 42;
        const PROBLEM_TITLE: &str = "not A Real Problem";
        type Solution = u32;
        const SOLUTION: Self::Solution = 42;
    }

    impl ProblemSolution for Solution00X {
        type Problem = Problem00X;
        
        fn solve() -> u32 {
            6 * 7
        }
    }

    struct Problem00Y;
    struct Solution00YA;
    struct Solution00YB;

    impl ProblemDescriptor for Problem00Y {
        const PROBLEM_ID: usize = 999;
        const PROBLEM_TITLE: &str = "thisIs4SureNotAProblem";
        type Solution = u64;
        const SOLUTION: Self::Solution = 1_234_321;
    }

    impl ProblemSolution for Solution00YA {
        type Problem = Problem00Y;
        const SOLUTION_ID: char = 'A';
        
        fn solve() -> <Self::Problem as ProblemDescriptor>::Solution {
            1_234_321
        }
    }

    impl ProblemSolution for Solution00YB {
        type Problem = Problem00Y;
        const SOLUTION_ID: char = 'B';
    
        fn solve() -> u64 {
            1111 * 1111
        }
    }

    #[test]
    fn problem_descriptors_work () {
        assert_eq!(Problem00X::get_descriptor_str(), "problem_042_not_a_real_problem");
        assert_eq!(Solution00X::get_descriptor_str(), "problem_042_not_a_real_problem_solution_A");

        assert_eq!(Problem00Y::get_descriptor_str(), "problem_999_this_is_4_sure_not_a_problem");
        assert_eq!(Solution00YA::get_descriptor_str(), "problem_999_this_is_4_sure_not_a_problem_solution_A");
        assert_eq!(Solution00YB::get_descriptor_str(), "problem_999_this_is_4_sure_not_a_problem_solution_B");
    }
}