//! Find the maximum product of N adjacent digits in a large number

pub const BIG_NUM_STR: &str = concat!(
  "73167176531330624919225119674426574742355349194934",
  "96983520312774506326239578318016984801869478851843",
  "85861560789112949495459501737958331952853208805511",
  "12540698747158523863050715693290963295227443043557",
  "66896648950445244523161731856403098711121722383113",
  "62229893423380308135336276614282806444486645238749",
  "30358907296290491560440772390713810515859307960866",
  "70172427121883998797908792274921901699720888093776",
  "65727333001053367881220235421809751254540594752243",
  "52584907711670556013604839586446706324415722155397",
  "53697817977846174064955149290862569321978468622482",
  "83972241375657056057490261407972968652414535100474",
  "82166370484403199890008895243450658541227588666881",
  "16427171479924442928230863465674813919123162824586",
  "17866458359124566529476545682848912883142607690042",
  "24219022671055626321111109370544217506941658960408",
  "07198403850962455444362981230987879927244284909188",
  "84580156166097919133875499200524063689912560717606",
  "05886116467109405077541002256983155200055935729725",
  "71636269561882670428252483600823257530420752963450"
);

// TODO See if there's room for improvement here. Can we keep a single running product?
//   Maybe we could divide digits out of the product as they leave the window? 
/// Iterate over every window of length N in nums and find the product
/// Split around 0 digit to prevent 0 product
#[must_use]
pub fn max_product_subslice (nums: &[u32], window_size: usize) -> u64 {
  nums
    .split(|&n| n == 0)
    .flat_map(|s| s.windows(window_size))
    .map(|w| w
      .iter()
      .map(|&n| <u64>::from(n))
      .product()
    )
    .max()
    .unwrap_or_default()
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum NumParseError {
  InvalidCharacterFound
}

/// # Errors
/// 
/// Returns an error if one of the characters in the string is not a valid digit.
pub fn numstring_to_numslice (numstr: &str) -> Result<Vec<u32>, NumParseError> {
  numstr
    .chars()
    .map(|c| c.to_digit(10))
    .collect::<Option<Vec<u32>>>()
    .ok_or(NumParseError::InvalidCharacterFound)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_numstring_to_numslice () {
    let s = "31415";
    let numsl = [3, 1, 4, 1, 5];
    numstring_to_numslice(s)
      .unwrap()
      .iter()
      .zip(numsl)
      .for_each(|(&result, expected)| assert_eq!(result, expected));
  }

  #[test]
  fn test_invalid_numstring_to_numslice () {
    let s = "31a415";
    numstring_to_numslice(s).expect_err("Result for invalid numstr should be Err");
  }

  #[test]
  fn test_four_window () {
    let s = BIG_NUM_STR;
    let nums = numstring_to_numslice(s)
      .unwrap();
    let result = max_product_subslice(&nums, 4);
    assert_eq!(result, 5832);
  }

  #[test]
  fn test_thirteen_window () {
    let s = BIG_NUM_STR;
    let nums = numstring_to_numslice(s)
      .unwrap();
    let result = max_product_subslice(&nums, 13);
    assert_eq!(result, 23_514_624_000);
  }
  
}