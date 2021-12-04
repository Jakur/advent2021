use super::*;

#[derive(Debug)]
struct Board {
    grid: [u8; 25],
    set: IntMap,
    row_count: [u8; Self::LENGTH],
    col_count: [u8; Self::LENGTH],
    solved: bool,
}

#[derive(Debug)]
struct IntMap {
    data: [u8; Self::MAX_VAL],
}

impl IntMap {
    const MAX_VAL: usize = 100;
    const EMPTY: u8 = 255;
    fn new() -> Self {
        Self {
            data: [Self::EMPTY; Self::MAX_VAL],
        }
    }
    fn insert(&mut self, index: usize, val: u8) {
        self.data[index] = val;
    }
    fn remove(&mut self, index: usize) -> Option<u8> {
        let val = self.data[index as usize];
        if val == Self::EMPTY {
            None
        } else {
            self.data[index] = Self::EMPTY;
            Some(val)
        }
    }
}

impl Board {
    const LENGTH: usize = 5;
    fn new(grid: [u8; 25], set: IntMap) -> Self {
        Self {
            grid,
            set,
            row_count: [0; Self::LENGTH],
            col_count: [0; Self::LENGTH],
            solved: false,
        }
    }
    fn update(&mut self, draw: u8) -> bool {
        if self.solved {
            return false;
        }
        if let Some(idx) = self.set.remove(draw as usize) {
            let idx = idx as usize;
            self.row_count[idx / Self::LENGTH] += 1;
            self.col_count[idx % Self::LENGTH] += 1;
            if self.row_count[idx / Self::LENGTH] == 5 || self.col_count[idx % Self::LENGTH] == 5 {
                self.solved = true;
                return true;
            }
        }
        false
    }
    fn sum_unmarked(&self) -> usize {
        self.set
            .data
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, x)| if x != IntMap::EMPTY { Some(i) } else { None })
            .sum()
    }
}

pub fn solve(input: &[u8]) -> Solution<u32, u32> {
    let mut order = Vec::new();
    let mut num = 0;
    let mut idx = 0;
    loop {
        match input[idx] {
            b'\n' => {
                order.push(num);
                idx += 2;
                break;
            }
            b',' => {
                order.push(num);
                num = 0;
            }
            _ => {
                num = num * 10 + (input[idx] - b'0');
            }
        }
        idx += 1;
    }

    let mut grid: [u8; 25] = [0; 25];
    let mut set = IntMap::new();
    let mut boards = Vec::new();
    let mut grid_index = 0;
    while idx < input.len() {
        let num = input[idx].saturating_sub(b'0') * 10 + input[idx + 1].saturating_sub(b'0');
        grid[grid_index] = num;
        set.insert(num as usize, grid_index as u8);
        grid_index += 1;
        idx += 3;
        if grid_index >= 25 {
            idx += 1;
            boards.push(Board::new(grid, set));
            grid_index = 0;
            grid = [0; 25];
            set = IntMap::new();
        }
    }
    let (part1, part2) = run_until_solved(&mut boards, &order);
    Solution::new(part1, part2)
}

fn run_until_solved(boards: &mut Vec<Board>, order: &[u8]) -> (u32, u32) {
    let mut part1 = 0;
    let mut solved = 0;
    let len = boards.len();
    for num in order.iter().copied() {
        for board in boards.iter_mut() {
            if board.update(num) {
                solved += 1;
                if part1 == 0 {
                    part1 = num as u32 * board.sum_unmarked() as u32;
                } else if solved == len {
                    return (part1, num as u32 * board.sum_unmarked() as u32);
                }
            }
        }
    }
    unimplemented!();
}
