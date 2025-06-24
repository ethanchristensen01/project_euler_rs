use std::ops::AddAssign;

pub struct CartesianIter <const N: usize> {
    curr: [usize; N],
    max: [usize; N],
    done: bool
}

impl<const N: usize> From<&[usize; N]> for CartesianIter<N> {
    fn from(value: &[usize; N]) -> Self {
        Self {
            curr: [0; N],
            max: *value,
            done: false
        }
    }
}

impl<const N: usize> CartesianIter<N> {
    #[must_use]
    pub fn new (size: &[usize; N]) -> Self {
        size.into()
    }
}

impl<const N: usize> Iterator for CartesianIter<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let next = self.curr;
        for i in 0..N {
            self.curr[i].add_assign(1);
            if self.curr[i] == self.max[i] {
                self.curr[i] = 0;
                // if i is on the last digit, this will make the state of self.curr weird
                // This is okay, because just mark it as done.
            } else {
                return Some(next);
            }
        }
        self.done = true;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_iter () {
        CartesianIter::new(&[3, 4, 5]).for_each(|p| {
            dbg!(p);
        });
    }
}