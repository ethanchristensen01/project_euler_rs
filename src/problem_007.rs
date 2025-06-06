//! Find the 10001st prime number
use crate::utils::prime::{PrimeSieve, Int};

/// ZERO INDEXED
#[must_use]
pub fn nth_prime(n: usize) -> Int {
  let mut prime_sieve = PrimeSieve::with_capacity(n);
  prime_sieve.nth(n).unwrap_or(2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_10_primes () {
    let prime_sieve = PrimeSieve::default();
    let actual_primes: Vec<Int> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    actual_primes.iter().zip(prime_sieve).for_each(|(&expected, actual)| {
      assert_eq!(expected, actual);
    });
  }

  #[test]
  fn check_2nd_prime () {
    assert_eq!(nth_prime(1), 3);
  }

  #[test]
  fn check_10001st_prime () {
    assert_eq!(nth_prime(10000), 104_743);
  }
}
