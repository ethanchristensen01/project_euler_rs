//! Largest Product in a Grid

use crate::Problem;
use std::{array, fmt::Debug};

const BIG_NUM_GRID: [[u32; 20]; 20] = [
    [
        8, 2, 22, 97, 38, 15, 0, 40, 0, 75, 4, 5, 7, 78, 52, 12, 50, 77, 91, 8,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 0,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 3, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 4, 60, 11, 42, 69, 24, 68, 56, 1, 32, 56, 71, 37, 2, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 3, 45, 2, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 5, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 9, 75, 0, 76, 44, 20, 45, 35, 14, 0, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 3, 80, 4, 62, 16, 14, 9, 53, 56, 92,
    ],
    [
        16, 39, 5, 42, 96, 35, 31, 47, 55, 58, 88, 24, 0, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 0, 48, 35, 71, 89, 7, 5, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 5, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 4, 89, 55, 40,
    ],
    [
        4, 52, 8, 83, 97, 35, 99, 16, 7, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 3, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        4, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 8, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 4, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 1, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 5, 54,
    ],
    [
        1, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 1, 89, 19, 67, 48,
    ],
];
pub struct Grid<const W: usize, const H: usize, T> {
    items: [[T; W]; H],
}

impl<const W: usize, const H: usize, T> Debug for Grid<W, H, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("W", &W)
            .field("H", &H)
            .finish()
    }
}

impl<const W: usize, const H: usize, T> Grid<W, H, T> {
    const fn get_2d(&self, (row, col): (usize, usize)) -> Option<&T> {
        if row >= H || col >= W {
            None
        } else {
            Some(&self.items[row][col])
        }
    }

    pub const fn new(items: [[T; W]; H]) -> Self {
        Self { items }
    }

    pub fn adjacency_iter<const N: usize>(&self) -> AdjacencyIterator<'_, N, W, H, T> {
        AdjacencyIterator {
            row: 0,
            col: 0,
            direction: Direction::default(),
            grid: self,
            is_new: true,
            is_done: false,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    DownRight,
    Down,
    DownLeft,
}

impl Default for Direction {
    fn default() -> Self {
        Self::const_default()
    }
}

impl Direction {
    const fn rotate(&self) -> Option<Self> {
        match self {
            Self::Right => Some(Self::DownRight),
            Self::DownRight => Some(Self::Down),
            Self::Down => Some(Self::DownLeft),
            Self::DownLeft => None,
        }
    }

    const fn const_default() -> Self {
        Self::Right
    }

    const fn offset(&self, pos: &(usize, usize), n: usize) -> (usize, usize) {
        match self {
            Self::Right => (pos.0, pos.1 + n),
            Self::DownRight => (pos.0 + n, pos.1 + n),
            Self::Down => (pos.0 + n, pos.1),
            Self::DownLeft => (pos.0 + n, pos.1 - n),
        }
    }
}

#[derive(Debug)]
pub struct AdjacencyIterator<'a, const N: usize, const W: usize, const H: usize, T> {
    row: usize,
    col: usize,
    direction: Direction,
    grid: &'a Grid<W, H, T>,
    is_new: bool,
    is_done: bool,
}

impl<'a, const N: usize, const W: usize, const H: usize, T> From<&'a Grid<W, H, T>>
    for AdjacencyIterator<'a, N, W, H, T>
{
    fn from(value: &'a Grid<W, H, T>) -> Self {
        Self {
            row: 0,
            col: 0,
            direction: Direction::default(),
            grid: value,
            is_new: true,
            is_done: false,
        }
    }
}

impl<'a, const N: usize, const W: usize, const H: usize, T: Debug>
    AdjacencyIterator<'a, N, W, H, T>
{
    fn adj_next(&mut self) -> Option<[&'a T; N]> {
        loop {
            if self.is_done {
                break None;
            } else if self.is_new {
                self.is_new = false;
            } else {
                self.adj_step();
            }
            if self.curr_adj_valid() {
                let result: [&'a T; N] = array::from_fn(|i| {
                    self.grid
                        .get_2d(self.direction.offset(&(self.row, self.col), i))
                        .expect("We should have already bounds checked")
                });
                if self.is_done {
                    break None;
                }
                break Some(result);
            }
        }
    }

    const fn curr_adj_valid(&self) -> bool {
        match self.direction {
            _ if self.row >= H || self.col >= W => false,
            Direction::Right if self.col <= W - N => true,
            Direction::DownRight if self.row <= H - N && self.col <= W - N => true,
            Direction::Down if self.row <= H - N => true,
            Direction::DownLeft if self.row <= H - N && self.col >= N - 1 => true,
            _ => false,
        }
    }

    #[inline]
    const fn adj_step(&mut self) {
        self.rot_step();
    }

    const fn rot_step(&mut self) {
        if let Some(d) = self.direction.rotate() {
            self.direction = d;
        } else {
            self.direction = Direction::const_default();
            self.col_step();
        }
    }

    const fn col_step(&mut self) {
        self.col += 1;
        if self.col >= W {
            self.col = 0;
            self.row_step();
        }
    }

    const fn row_step(&mut self) {
        self.row += 1;
        if self.row >= H {
            self.row = 0;
            self.is_done = true;
        }
    }
}

impl<'a, const N: usize, const W: usize, const H: usize, T: Debug> Iterator
    for AdjacencyIterator<'a, N, W, H, T>
{
    type Item = [&'a T; N];

    fn next(&mut self) -> Option<Self::Item> {
        self.adj_next()
    }
}

pub struct Problem011;

impl Problem for Problem011 {
    type Solution = u32;

    fn name() -> &'static str {
        "011 Largest Product in a Grid"
    }

    fn solve() -> Self::Solution {
        Grid::new(BIG_NUM_GRID)
            .adjacency_iter::<4>()
            .map(|a| a.iter().copied().product())
            .max()
            .expect("adj iter should not be empty")
    }

    fn is_correct(solution: &Self::Solution) -> bool {
        *solution == 70_600_674
    }
}

#[cfg(test)]
mod tests {
    const DEBUG_GRID_5_5: [[i32; 5]; 5] = [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
        [21, 22, 23, 24, 25],
    ];
    // 15 + 15 + 9 + 9
    const DEBUG_5_5_ADJACENCIES: &[[i32; 3]] = &[
        [1, 2, 3],
        [1, 7, 13],
        [1, 6, 11],
        [2, 3, 4],
        [2, 8, 14],
        [2, 7, 12],
        [3, 4, 5],
        [3, 9, 15],
        [3, 8, 13],
        [3, 7, 11],
        [4, 9, 14],
        [4, 8, 12],
        [5, 10, 15],
        [5, 9, 13],
        [6, 7, 8],
        [6, 12, 18],
        [6, 11, 16],
        [7, 8, 9],
        [7, 13, 19],
        [7, 12, 17],
        [8, 9, 10],
        [8, 14, 20],
        [8, 13, 18],
        [8, 12, 16],
        [9, 14, 19],
        [9, 13, 17],
        [10, 15, 20],
        [10, 14, 18],
        [11, 12, 13],
        [11, 17, 23],
        [11, 16, 21],
        [12, 13, 14],
        [12, 18, 24],
        [12, 17, 22],
        [13, 14, 15],
        [13, 19, 25],
        [13, 18, 23],
        [13, 17, 21],
        [14, 19, 24],
        [14, 18, 22],
        [15, 20, 25],
        [15, 19, 23],
        [16, 17, 18],
        [17, 18, 19],
        [18, 19, 20],
        [21, 22, 23],
        [22, 23, 24],
        [23, 24, 25],
    ];

    use std::collections::HashSet;

    use super::*;

    #[test]
    fn debug() {
        let grid = Grid::new(DEBUG_GRID_5_5);
        let adj_set = grid.adjacency_iter::<3>().collect::<HashSet<[&i32; 3]>>();
        assert!(
            DEBUG_5_5_ADJACENCIES
                .iter()
                .all(|adj| adj_set.contains(&adj.each_ref())),
            "Some adjacencies were missed"
        );
        assert_eq!(
            adj_set.len(),
            DEBUG_5_5_ADJACENCIES.len(),
            "There were extra adjacencies reported"
        );
    }

    #[test]
    fn problem_011_solve() {
        assert!(Problem011::test());
    }
}
