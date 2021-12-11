use super::*;

const ROWS: usize = 10;
const COLS: usize = 10;
const EXPLODE: u8 = 10;

#[derive(Debug, Clone, Copy)]
struct Slot {
    row: usize,
    col: usize,
}

impl Slot {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct Neighbors {
    row: usize,
    col: usize,
    iter_count: u8,
}

impl Neighbors {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            iter_count: 0,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Slot;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let neighbor = match self.iter_count {
            0 => (self.row.wrapping_sub(1), self.col.wrapping_sub(1)),
            1 => (self.row.wrapping_sub(1), self.col),
            2 => (self.row.wrapping_sub(1), self.col + 1),
            3 => (self.row, self.col.wrapping_sub(1)),
            4 => (self.row, self.col + 1),
            5 => (self.row + 1, self.col.wrapping_sub(1)),
            6 => (self.row + 1, self.col),
            7 => (self.row + 1, self.col + 1),
            _ => return None,
        };
        self.iter_count += 1;
        if let Some(x) = Grid::index(neighbor.0, neighbor.1) {
            Some(x)
        } else {
            self.next() // We need to keep checking the other values
        }
    }
}

struct Grid {
    data: [u8; ROWS * COLS],
    flashes: u32,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            data: [0; ROWS * COLS],
            flashes: 0,
        }
    }
    pub fn neighbors(row: usize, col: usize) -> impl Iterator<Item = Slot> {
        Neighbors::new(row, col)
    }
    pub fn index(row: usize, col: usize) -> Option<Slot> {
        if row < ROWS && col < COLS {
            Some(Slot::new(row, col))
        } else {
            None
        }
    }
    pub fn increment_all(&mut self) {
        for x in self.data.iter_mut() {
            *x += 1;
        }
    }
    pub fn print_grid(&self) {
        for row in 0..ROWS {
            println!("{:?}", &self.data[row * COLS..row * COLS + COLS]);
        }
    }
}

pub fn solve(input: &[u8]) -> Solution<u32, u32> {
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
    let mut stack = Vec::new();
    for _ in 0..100 {
        do_step(&mut grid, &mut stack);
    }
    // My solution doesn't resolve the explosions until the next round, so count them here
    let part1 = grid.flashes + grid.data.iter().filter(|&&x| x >= 10).count() as u32;
    let mut step = 100;
    // Just assume part2 has to solve after part1
    let part2 = loop {
        step += 1; // 1 indexing, so we increment before
        if do_step(&mut grid, &mut stack) {
            break step;
        }
    };
    Solution::new(part1, part2)
}

fn do_step(grid: &mut Grid, stack: &mut Vec<Slot>) -> bool {
    const ALL: u32 = (ROWS * COLS) as u32;
    let last_flashes = grid.flashes;
    grid.increment_all();
    for row in 0..ROWS {
        for col in 0..COLS {
            handle_explosion(grid, stack, row, col);
        }
    }
    while let Some(slot) = stack.pop() {
        handle_explosion(grid, stack, slot.row, slot.col);
    }
    let diff = grid.flashes - last_flashes;
    diff == ALL
}

fn handle_explosion(grid: &mut Grid, stack: &mut Vec<Slot>, row: usize, col: usize) {
    if grid.data[row * COLS + col] >= EXPLODE {
        grid.flashes += 1;
        grid.data[row * COLS + col] = 0;
        let neighbors = Grid::neighbors(row, col);
        for neighbor in neighbors {
            let val = grid.data[neighbor.row * COLS + neighbor.col];
            if val == 0 || val >= EXPLODE {
                continue;
            }
            grid.data[neighbor.row * COLS + neighbor.col] += 1;
            if grid.data[neighbor.row * COLS + neighbor.col] == EXPLODE {
                stack.push(neighbor);
            }
        }
    }
}
