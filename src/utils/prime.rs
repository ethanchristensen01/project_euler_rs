use std::{cell::RefCell, collections::BTreeMap};
#[derive(Default)]
pub struct PrimeSieve {
    primes: RefCell<Vec<u64>>,
}

impl PrimeSieve {
    pub fn reserve_exact(&self, additional: usize) {
        self.primes.borrow_mut().reserve_exact(additional);
    }

    #[must_use]
    pub fn to_vec(self) -> Vec<u64> {
        self.primes.into_inner()
    }

    #[must_use]
    pub fn get_vec(&self) -> Vec<u64> {
        self.primes.clone().into_inner()
    }

    #[must_use]
    pub fn first_n_primes(n: usize) -> Self {
        let prime_iter = Self::default();
        prime_iter.grow_n(n);
        prime_iter
    }

    pub fn grow(&self) -> u64 {
        let next = self
            .primes
            .borrow()
            .last()
            .and_then(|&prev| {
                #[expect(clippy::maybe_infinite_iter)]
                ((prev + 1)..).find(|n| {
                    let nsqrt = n.isqrt();
                    self.primes.borrow()
                        .iter()
                        .take_while(|p| nsqrt.ge(p))
                        .all(|p| n % p != 0)
                })
            })
            .unwrap_or(2);
        self.primes.borrow_mut().push(next);
        next
    }

    pub fn grow_n(&self, n: usize) {
        self.primes.borrow_mut().reserve(n);
        for _ in 0..n {
            self.grow();
        }
    }

    pub fn grow_until(&self, n: u64) {
        if self.primes.borrow().last().is_some_and(|&p| p <= n) {
            return
        }
        while self.grow() < n {}
    }

    pub fn grow_until_sqrt(&mut self, n: u64) {
        self.grow_until(n.isqrt());
    }

    pub fn iter(&self) -> PrimeSieveIterator {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a PrimeSieve {
    type Item = u64;

    type IntoIter = PrimeSieveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PrimeSieveIterator {
            sieve: self,
            index: 0
        }
    }
}

pub struct PrimeSieveIterator<'a> {
    sieve: &'a PrimeSieve,
    index: usize
}

impl Iterator for PrimeSieveIterator<'_> {
    type Item = u64;
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.sieve.primes.borrow_mut().get(self.index).copied();
        self.index += 1;
        result.or_else(|| Some(self.sieve.grow()))
    }
}

#[derive(Debug, Default)]
pub struct PrimeFactorization (
    BTreeMap<u64, usize>
);

impl PrimeFactorization {
    #[must_use]
    pub const fn from_btreemap(map: BTreeMap<u64, usize>) -> Self  {
        Self(map)
    }

    pub fn from_slice(slice: &[usize], primes: &PrimeSieve) -> Self  {
        Self (
            primes
                .iter()
                .zip(slice.to_owned())
                .collect()
        )
    }

    fn add_factor(&mut self, n: u64) {
        self.0.entry(n)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    /// # Panics
    /// Panics if one of the prime factors has a count of greater than `u64::MAX`
    #[must_use]
    pub fn full_number(&self) -> u64 {
        self.0
            .iter()
            .fold(1, |initial , (factor, &count)|
                initial * factor.pow(count.try_into().expect("Prime factor count was not expected to exceed u32 maximum"))
            )
    }

    /// # Panics
    /// Panics if one of the prime factors has a count of greater than `u64::MAX`
    #[must_use]
    pub fn count_factors(&self) -> u64 {
        self.0
            .iter()
            .fold(1_u64, |initial , (_, &count)|
                u64::try_from(count + 1).expect("Prime factor count was not expected to exceed u32 maximum") * initial
            )
    }

    #[must_use]
    pub fn multiply_factors (&self, rhs: &Self) -> Self {
        let mut new_map = self.0.clone();
        for (r_factor, r_count) in rhs.0.clone() {
            new_map
                .entry(r_factor)
                .and_modify(|l_count| *l_count += r_count)
                .or_insert(r_count);
        }
        Self(new_map)
    }

    /// # Errors
    /// Fails if factor was not found or if it was already 0
    pub fn div_factor (&mut self, rhs: u64) -> Result<(), ()> {
        self.0
            .get_mut(&rhs)
            .and_then(|e| if e == &0 {
                None
            } else {
                *e -= 1;
                Some(())
            })
            .ok_or(())
    }
}

impl PartialEq for PrimeFactorization {
    fn eq(&self, other: &Self) -> bool {
        let s = self.0
            .iter()
            .filter(|(factor, count)| factor.gt(&&1) && count.ge(&&1));

        let o = other.0
            .iter()
            .filter(|(factor, count)| factor.gt(&&1) && count.ge(&&1));

        s.eq(o)
    }
}

#[must_use]
pub fn prime_factorize (number: u64, primes: &PrimeSieve) -> PrimeFactorization {
    let mut number = number;
    let mut prime_factors = PrimeFactorization::default();
    if number == 0 {
        return prime_factors
    }
    for p in primes {
        while number % p == 0 {
            prime_factors.add_factor(p);
                
            number /= p;
        }
        if number == 1 {
            break
        }
    };
    prime_factors
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! prime_factorization {
        ( $( $factor:expr => $count:expr ),* $(,)? ) => {{
            #[allow(unused_mut, reason="sometimes map may be empty")]
            let mut map = std::collections::BTreeMap::new();
            $(
                if $count > 0 {
                    map.insert($factor, $count);
                }
            )*
            $crate::utils::prime::PrimeFactorization::from_btreemap(map)
        }};
    }

    #[test]
    fn check_10_primes() {
        let primes = PrimeSieve::first_n_primes(10).to_vec();
        let actual_primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        actual_primes
            .iter()
            .zip(primes)
            .for_each(|(&expected, actual)| {
                assert_eq!(expected, actual);
            });
    }

    #[test]
    fn check_prime_factorization() {
        let primes = PrimeSieve::default();
        
        assert_eq!(prime_factorize(42, &primes), prime_factorization!(
            2 => 1,
            3 => 1,
            7 => 1
        ));

        assert_eq!(prime_factorize(8316, &primes), prime_factorization!(
            2 => 2,
            3 => 3,
            7 => 1,
            11 => 1
        ));
    }

    #[test]
    fn check_factorization_eq () {
        assert_eq!(prime_factorization!(
            2 => 2,
            3 => 0
        ), prime_factorization!(
            2 => 2
        ), "Should ignore factor with count 0");

        assert_eq!(prime_factorization!(
            1 => 5,
            2 => 2,
        ), prime_factorization!(
            2 => 2
        ), "Should ignore factor 1 with any count");
    }

    #[test]
    fn check_8316_factorization_test () {
        let n: u64 = 8317;
        let primes = PrimeSieve::default();
        let factors = prime_factorize(n, &primes);
        assert_eq!(factors, prime_factorization!(
            n => 1
        ));
    }

    #[test]
    fn check_multiply_factors() {
        let a = prime_factorization!(
            2 => 2,
            3 => 2
        );

        let b = prime_factorization!(
            2 => 1,
            5 => 1
        );

        let c = a.multiply_factors(&b);

        assert_eq!(c, prime_factorization!(
            2 => 3,
            3 => 2,
            5 => 1
        ));
    }

        #[test]
    fn check_div_factors() {
        let mut a = prime_factorization!(
            2 => 2,
            3 => 2
        );

        a.div_factor(2).expect("Should not error yet");

        assert_eq!(a, prime_factorization!(
            2 => 1,
            3 => 2
        ));

        a.div_factor(2).expect("Should not error yet");

        assert_eq!(a, prime_factorization!(
            3 => 2
        ));

        a.div_factor(2).expect_err("Should error because there are no remaining 2 factors");
    }

    #[test]
    fn check_factorize_0() {
        let primes = PrimeSieve::default();
        assert_eq!(
            prime_factorize(0, &primes),
            prime_factorization!()
        );
    }
}
