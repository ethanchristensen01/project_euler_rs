//! Find the only pythagorean triple which sums to 1000, and return the product of each number

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
    fn test_find_pythagorean_triple_1000() {
        assert_eq!(find_pythagorean_triple_1000(), Some(31_875_000));
    }
}
