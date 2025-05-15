pub mod problem_006;
pub mod problem_007;
pub mod problem_008;
pub mod problem_009;
pub mod problem_010;

pub trait Problem {
  type Solution: Eq + std::fmt::Debug;
  fn name() -> &'static str;
  fn solve() -> Self::Solution;
  fn is_correct(solution: &Self::Solution) -> bool;

  #[must_use]
  fn test() -> bool {
    Self::is_correct(&Self::solve())
  }
}
