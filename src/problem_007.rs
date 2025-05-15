//! Find the 10001st prime number
pub type Int = u64;

#[derive(Default)]
pub struct PrimeSieve {
  primes: Vec<Int>
}

impl PrimeSieve {
  pub fn reserve_exact(&mut self, additional: usize) {
    self.primes.reserve_exact(additional);
  }

  #[must_use]
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      primes: Vec::with_capacity(capacity)
    }
  }

  #[must_use]
  pub fn to_vec(self) -> Vec<Int> {
    self.primes
  }
}

impl Iterator for PrimeSieve {
  type Item = Int;

  #[allow(clippy::maybe_infinite_iter)]
  fn next(&mut self) -> Option<Self::Item> {
    let next = self.primes.last()
      .and_then(|&prev| ((prev + 1)..)
        .find(|n| {
          let nsqrt = n.isqrt();
          self.primes.iter()
            .take_while(|p| nsqrt.ge(p))
            .all(|p| n % p != 0)
        })
      )
      .unwrap_or(2);
    self.primes.push(next);
    Some(next)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (usize::MAX, None)
  }
}

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