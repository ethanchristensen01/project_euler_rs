#[derive(Debug)]
pub struct Collatz {
    data: Vec<Option<u64>>,
    len_records: Vec<(usize, u64)>,
    pub best: (usize, u64),
    max_num: usize
}

#[non_exhaustive]
#[derive(Debug)]
pub enum CollatzError {
    IntegerOverflow (usize)
}
pub trait CollatzCore {
    fn next_empty_num (&self) -> usize;
    /// Get the length of the sequence starting from num
    fn get_seq_length_from (&self, num: usize) -> Option<u64>;
    fn set_cache_length_at (&mut self, num: usize, length: u64);

    /// Given some number, find a number after it in the sequence and the size of its sequence
    fn collatz_reduce (&mut self, num: usize) -> Result<(usize, u64), CollatzError>;

}

impl CollatzCore for Collatz {
    fn next_empty_num (&self) -> usize {
        self.data.iter().enumerate().find(|(_, v)| v.is_none()).map_or_else(|| self.data.len() + 1, |(i,_)| i + 1)
    }

    fn get_seq_length_from (&self, num: usize) -> Option<u64> {
        self.data.get(num - 1).copied().flatten()
    }

    fn set_cache_length_at (&mut self, num: usize, length: u64) {
        if length > self.best.1 {
            self.len_records.push(self.best);
            self.best = (num, length);
        }
        if self.data.len() < num {
            self.data.resize(num, None);
        }
        self.data[num - 1] = Some(length);
    }

    fn collatz_reduce (&mut self, num: usize) -> Result<(usize, u64), CollatzError> {
        if let Some(length) = self.get_seq_length_from(num) {
            return Ok((num, length));
        }

        let original_num = num;

        let mut stack = vec![];
        let mut num = num;
        let mut count = loop {
            stack.push(num);
            if num <= 1 {
                break num as u64
            } else if let Some(length) = self.get_seq_length_from(num) {
                break length
            } else if num.is_multiple_of(2) {
                num /= 2;
            } else {
                num = num * 3 + 1;
            }
        };
        for n in stack.into_iter().rev() {
            if n <= 1_000_000 {
                self.set_cache_length_at(n, count);
            }
            count += 1;
        };
        Ok((original_num, count - 1))
    }
}

impl Collatz {
    pub fn new () -> Self {
        Self::with_capacity(1_000_000)
    }

    pub fn with_capacity (max_num: usize) -> Self {
        let mut data = Vec::with_capacity(max_num);
        data.push(Some(1));
        Self { data, len_records: vec![(1, 1)], best: (1, 1), max_num }
    }

    pub fn run (&mut self) {
        for i in 1..1_000_000 {
            self.collatz_reduce(i).expect("Failed to find something I guess");
        }
    }

    fn print_lengths (&self, count: usize) {
        self.data.iter().take(count).enumerate().filter(|(_, v)| v.is_some()).for_each(|(i, v)| {
            println!("{}: length {}", i + 1, v.unwrap());
        });
    }

    fn print_records (&self) {
        for (num, length) in self.len_records.clone() {
            println!("{num}: length {length}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::collatz::*;

    #[test]
    fn debugging () {
        let mut c = Collatz::with_capacity(1_000_000);
        for i in 1..1_000_000 {
            c.collatz_reduce(i).expect("Failed to find something I guess");
        }
        println!("Best: {} with length {}", c.best.0, c.best.1);
        c.print_lengths(100);
    }
}