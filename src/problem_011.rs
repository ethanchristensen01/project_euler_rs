//! Largest Product in a Grid

use std::{array, fmt::Debug};

use crate::Problem;
struct Problem011;

pub struct Grid<const W: usize, const H: usize, T> {
  items: [[T; W]; H]
}

impl <const W: usize, const H: usize, T> Debug for Grid<W, H, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
          .field("W", &W)
          .field("H", &H)
          .finish()
    }
}

impl<const W: usize, const H: usize, T>  Grid<W, H, T> {
  const fn get_2d (&self, (row, col): (usize, usize)) -> Option<&T> {
    if row >= H || col >= W {
      None
    } else {
      Some(&self.items[row][col])
    }
  }

  pub const fn new (items: [[T; W]; H]) -> Self {
    Self {items}
  }

  pub fn diag_iter<const N: usize>(&self) -> DiagonalIterator<'_, N, W, H, T> {
    DiagonalIterator { row: 0, col: 0, direction: Direction::default(), grid: self, is_new: true, is_done: false }
  }
}


#[derive(Debug)]
enum Direction {
  DownRight,
  DownLeft
}

impl Default for Direction {
  fn default() -> Self {
    Self::const_default()
  }
}

impl Direction {
  const fn rotate (&self) -> Option<Self> {
    match self {
      Self::DownRight => Some(Self::DownLeft),
      Self::DownLeft => None
    }
  }

  const fn const_default() -> Self {
    Self::DownRight
  }

  const fn offset(&self, pos: &(usize, usize), n: usize) -> (usize, usize) {
    match self {
      Self::DownRight => (pos.0 + n, pos.1 + n),
      Self::DownLeft => (pos.0 + n, pos.1 - n)
    }
  }
}

#[derive(Debug)]
pub struct DiagonalIterator <'a, const N: usize, const W: usize, const H: usize, T> {
  row: usize,
  col: usize,
  direction: Direction,
  grid: &'a Grid<W, H, T>,
  is_new: bool,
  is_done: bool
}

impl <'a, const N: usize, const W: usize, const H: usize, T> From<&'a Grid<W, H, T>> for DiagonalIterator<'a, N, W, H, T> {
    fn from(value: &'a Grid<W, H, T>) -> Self {
        Self { row: 0, col: 0, direction: Direction::default(), grid: value, is_new: true, is_done: false }
    }
}

impl <'a, const N: usize, const W: usize, const H: usize, T: Debug> DiagonalIterator<'a, N, W, H, T> {
  fn diag_next (&mut self) -> Option<[&'a T; N]> {
    loop {
      if self.is_done {
        break None
      } else if self.is_new {
        self.is_new = false;
      } else {
        self.diag_step();
      }
      if self.curr_diag_valid() {
        let result: [&'a T; N] = array::from_fn(|i|
          self.grid.get_2d(
            self.direction.offset(&(self.row, self.col), i)
          ).expect("We should have already bounds checked")
        );
        break Some(result);
      }
    }
  }

  const fn curr_diag_valid (&self) -> bool {
    match self.direction {
      _ if self.row >= H || self.col >= W
        => false,
      Direction::DownRight if self.row <= H - N && self.col <= W - N
        => true,
      Direction::DownLeft if self.row <= H - N && self.col >= N - 1
        => true,
      _ => false
    }
  }

  #[inline]
  const fn diag_step (&mut self) {
    self.rot_step();
  }
  
  const fn rot_step (&mut self) {
    if let Some(d) = self.direction.rotate() {
      self.direction = d;
    } else {
      self.direction = Direction::const_default();      
      self.col_step();
    }
  }

  const fn col_step (&mut self) {
    self.col += 1;
    if self.col >= W {
      self.col = 0;
      self.row_step();
    }
  }

  const fn row_step (&mut self) {
    self.row += 1;
    if self.row >= H {
      self.row = 0;
      self.is_done = true;
    }
  }
}

impl<'a, const N: usize, const W: usize, const H: usize, T: Debug> Iterator for DiagonalIterator<'a, N, W, H, T> {
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
      self.diag_next()
    }
}


impl Problem for Problem011 {
  type Solution = u64;

  fn name() -> &'static str {
    "011 Largest Product in a Grid"
  }

  fn solve() -> Self::Solution {
    todo!()
  }

  fn is_correct(solution: &Self::Solution) -> bool {
    dbg!(solution);
    todo!()
  }
}


#[cfg(test)]
mod tests {
  const DEBUG_GRID: [[i32; 5]; 5] = [
    [ 1,  2,  3,  4,  5],
    [ 6,  7,  8,  9, 10],
    [11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20],
    [21, 22, 23, 24, 25]
  ];
  use super::*;

  #[test]
  fn debug () {
    let a = Grid::new(DEBUG_GRID);
    a.diag_iter::<3>().take(10).for_each(|d| {dbg!(d);});
  }
}