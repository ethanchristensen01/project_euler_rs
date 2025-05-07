pub type Int = u32;

pub fn is_palindrome(n: Int) -> bool {
  let mut num: u64 = n.into();
  let mut rev = 0u64;
  while num > 0 {
    rev = rev * 10 + num % 10;
    num /= 10;
  }
  rev == n.into()
}

const A: Int = 900;
const D_ARR: [Int; 2] = [900, 800];
const B_ARR: [Int; 10] = [90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
const E_ARR: [Int; 10] = B_ARR;
const CF_ARR: [(Int, Int); 4] = [(9, 1), (7, 7), (3, 3), (1, 9)];
/// Assume digits = 3
pub fn find_palindrome_product () -> Option<Int> {
  for d in D_ARR {
    for b in B_ARR {
      for e in E_ARR {
        for (c, f) in CF_ARR {
          let num = (A + b + c) * (d + e + f);
          if is_palindrome(num) {
            return Some(num)
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
  fn find_palindrome_product_correct () {
    assert_eq!(Some(906609), find_palindrome_product())
  }

  #[test]
  fn is_palindrome_correct () {
    let inputs: Vec<(Int, bool)> = vec!(
      (123, false),
      (121, true),
      (123456, false),
      (123326, false),
      (123351, false),
      (123421, false),
      (123321, true)
    );
    inputs.into_iter().for_each(|input| assert_eq!(is_palindrome(input.0), input.1));
  }
}