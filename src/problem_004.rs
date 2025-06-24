//! Find the largest palindrome product of a pair of 3 digit numbers

use crate::{ProblemDescriptor, ProblemSolution};

type Int = u32;
pub struct Problem004;

impl ProblemDescriptor for Problem004 {
    const PROBLEM_ID: usize = 4;
    const PROBLEM_TITLE: &str = "largest palindrome product";
    type Solution = Int;
    const SOLUTION: Self::Solution = 906_609;
}

pub struct Solution004;

impl ProblemSolution for Solution004 {
    type Problem = Problem004;

    fn solve() -> Int {
        find_palindrome_product_3().expect("solution should exist")
    }
}

#[inline]
#[must_use]
pub const fn is_palindrome(n: Int) -> bool {
    let mut num: Int = n;
    let mut rev: Int = 0;
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    rev == n
}

const A: Int = 900;
const D_ARR: [Int; 2] = [900, 800];
const B_ARR: [Int; 10] = [90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
const E_ARR: [Int; 10] = B_ARR;
const CF_ARR: [(Int, Int); 4] = [(9, 1), (7, 7), (3, 3), (1, 9)];
/// Assume num digits = 3
#[inline]
#[must_use]
fn find_palindrome_product_3() -> Option<Int> {
    for d in D_ARR {
        for b in B_ARR {
            for e in E_ARR {
                for (c, f) in CF_ARR {
                    let num_1 = A + b + c;
                    let num_2 = d + e + f;

                    if num_1 % 11 != 0 && num_2 % 11 != 0 {
                        continue;
                    }

                    let prod = num_1 * num_2;
                    if is_palindrome(prod) {
                        return Some(prod);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_004_solve() {
        assert!(Solution004::test());
    }

    #[test]
    fn is_palindrome_correct() {
        let inputs: Vec<(Int, bool)> = vec![
            (123, false),
            (121, true),
            (123_456, false),
            (123_326, false),
            (123_351, false),
            (123_421, false),
            (123_321, true),
        ];
        inputs
            .into_iter()
            .for_each(|input| assert_eq!(is_palindrome(input.0), input.1));
    }

    fn list_checked_numbers() -> Int {
        let mut count = 0;
        for d in D_ARR {
            for b in B_ARR {
                for e in E_ARR {
                    for (c, f) in CF_ARR {
                        let num_1 = A + b + c;
                        let num_2 = d + e + f;

                        if num_1 % 11 != 0 && num_2 % 11 != 0 {
                            continue;
                        }

                        let prod = num_1 * num_2;
                        println!("checked {num_1} () * {num_2} = {prod}");
                        count += 1;
                        if is_palindrome(prod) {
                            return count;
                        }
                    }
                }
            }
        }
        count
    }

    #[test]
    fn list_checked_numbers_dbg() {
        let num_checked = list_checked_numbers();
        println!("checked only {num_checked} products");
    }
}
