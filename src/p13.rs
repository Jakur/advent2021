use super::*;

use std::collections::HashSet;

enum Flip {
    Vertical(usize),
    Horizontal(usize),
}

struct Grid {
    data: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        let data = HashSet::with_capacity(2048);
        Self { data, rows, cols }
    }
    fn mark(&mut self, row: usize, col: usize) {
        self.data.insert((row, col));
        // self.data[row * self.cols + col] = 1;
    }
    fn h_flip_all(&mut self, num: usize) {
        for (x, y) in self.data.clone() {
            self.h_flip(x, y, num);
        }
    }
    fn v_flip_all(&mut self, num: usize) {
        for (x, y) in self.data.clone() {
            self.v_flip(x, y, num);
        }
    }
    fn h_flip(&mut self, row: usize, col: usize, x_line: usize) {
        if col > x_line {
            let diff = col - x_line;
            if let Some(mark_col) = x_line.checked_sub(diff) {
                self.data.insert((row, mark_col));
            }
            // self.data.insert((row, mark_col));
            // self.data[row * self.cols + mark_col] = 1;
        }
    }
    fn v_flip(&mut self, row: usize, col: usize, y_line: usize) {
        if row > y_line {
            let diff = row - y_line;
            if let Some(mark_row) = y_line.checked_sub(diff) {
                self.data.insert((mark_row, col));
            }
        }
    }
    fn set_dim(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
    }
    fn count(&self) -> u32 {
        let mut count = 0;
        for key in self.data.iter() {
            if key.0 < self.rows && key.1 < self.cols {
                count += 1;
            }
        }
        count
    }
    fn retain_live(&mut self) {
        let max_rows = self.rows;
        let max_cols = self.cols;
        self.data
            .retain(|(row, col)| *row < max_rows && *col < max_cols);
    }
    fn live_points(&self) -> Vec<&(usize, usize)> {
        self.data
            .iter()
            .filter(|(row, col)| *row < self.rows && *col < self.cols)
            .collect()
    }
}

pub fn solve(input: &str) -> Result<Solution<u32, u32>> {
    let mut vec = Vec::new();
    let mut flips = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        if line == "" {
            continue;
        }
        if line.starts_with("f") {
            let num = line.split("=").nth(1).unwrap().parse()?;
            if line.contains("x") {
                flips.push(Flip::Horizontal(num));
            } else {
                flips.push(Flip::Vertical(num));
            }
        } else {
            let mut sp = line.split(",");
            let x = sp.next().unwrap().parse::<usize>()?;
            if x > max_x {
                max_x = x;
            }
            let y = sp.next().unwrap().parse::<usize>()?;
            if y > max_y {
                max_y = y;
            }
            vec.push((x, y));
        }
    }
    let mut grid = Grid::new(max_y + 1, max_x + 1);
    for (x, y) in vec.into_iter() {
        grid.mark(y, x);
    }
    let mut part1 = 0;
    for (f_count, flip) in flips.into_iter().enumerate() {
        match flip {
            Flip::Horizontal(num) => {
                grid.h_flip_all(num);
                grid.set_dim(grid.rows, num);
            }
            Flip::Vertical(num) => {
                grid.v_flip_all(num);
                grid.set_dim(num, grid.cols);
            }
        }
        if f_count == 0 {
            part1 = grid.count();
        }
        grid.retain_live();
    }
    let mut text_grid = vec![' '; grid.rows * grid.cols];
    for (row, col) in grid.live_points() {
        text_grid[row * grid.cols + col] = '#';
    }
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            print!("{}", text_grid[row * grid.cols + col]);
        }
        println!();
    }

    Ok(Solution::new(part1, 0))
}
