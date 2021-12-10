use super::*;

const ROWS: usize = 100;
const COLS: usize = 100;

#[derive(Debug, Clone, Copy)]
struct Slot {
    row: usize,
    col: usize,
    val: u8,
}

impl Slot {
    fn new(row: usize, col: usize, val: u8) -> Self {
        Self { row, col, val }
    }
}

struct Neighbors<'a> {
    grid: &'a Grid,
    row: usize,
    col: usize,
    iter_count: u8,
}

impl<'a> Neighbors<'a> {
    fn new(grid: &'a Grid, row: usize, col: usize) -> Self {
        Self {
            grid,
            row,
            col,
            iter_count: 0,
        }
    }
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Slot;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let neighbor = match self.iter_count {
            0 => (self.row.wrapping_sub(1), self.col),
            1 => (self.row + 1, self.col),
            2 => (self.row, self.col.wrapping_sub(1)),
            3 => (self.row, self.col + 1),
            _ => return None,
        };
        self.iter_count += 1;
        if let Some(x) = self.grid.index(neighbor) {
            Some(x)
        } else {
            self.next() // We need to keep checking the other values
        }
    }
}

struct Grid {
    data: [u8; ROWS * COLS],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            data: [0; ROWS * COLS],
        }
    }
    pub fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Slot> + '_ {
        Neighbors::new(&self, row, col)
    }
    pub fn low_point(&self, row: usize, col: usize) -> bool {
        let grid_val = self.data[row * COLS + col];
        self.neighbors(row, col).all(|x| grid_val < x.val)
    }
    pub fn index(&self, index: (usize, usize)) -> Option<Slot> {
        if index.0 < ROWS && index.1 < COLS {
            Some(Slot::new(
                index.0,
                index.1,
                self.data[index.0 * COLS + index.1],
            ))
        } else {
            None
        }
    }
}

fn flood_fill(grid: &Grid, row: usize, col: usize, discovered: &mut Grid) -> usize {
    let mut size = 0;
    let mut stack = vec![grid.index((row, col)).unwrap()];
    while let Some(x) = stack.pop() {
        size += 1;
        for neighbor in grid.neighbors(x.row, x.col).filter(|x| x.val != 9) {
            let val = &mut discovered.data[neighbor.row * COLS + neighbor.col];
            if *val != 0 {
                continue;
            }
            *val += 1;
            stack.push(neighbor);
        }
    }
    size
}

pub fn solve(input: &[u8]) -> Solution<u32, usize> {
    let mut row = 0;
    let mut col = 0;
    let mut grid = Grid::new();
    for x in input.iter().copied() {
        if x == b'\n' {
            row += 1;
            col = 0;
        } else {
            debug_assert!(col < COLS);
            grid.data[row * COLS + col] = x - b'0';
            col += 1;
        }
    }
    let mut part1 = 0;
    let mut discovered = Grid::new();
    let mut counts = Vec::new();
    for row in 0..ROWS {
        for col in 0..COLS {
            if grid.data[row * COLS + col] == 9 {
                continue; // Cannot satisfy part 1 or 2
            }
            // Flood fill
            let check = &mut discovered.data[row * COLS + col];
            if *check == 0 {
                *check += 1;
                counts.push(flood_fill(&grid, row, col, &mut discovered));
            }
            if grid.low_point(row, col) {
                part1 += (grid.data[row * COLS + col] + 1) as u32;
            }
        }
    }
    counts.sort();
    Solution::new(part1, counts.into_iter().rev().take(3).product())
}
