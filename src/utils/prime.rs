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