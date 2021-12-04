use super::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Board {
    id: usize,
    grid: [u32; 25],
    set: IntMap,
    row_count: [u32; Self::LENGTH],
    col_count: [u32; Self::LENGTH],
    solved: bool,
}

#[derive(Debug)]
struct IntMap {
    data: [usize; Self::MAX_VAL],
}

impl IntMap {
    const MAX_VAL: usize = 100;
    const EMPTY: usize = 255;
    fn new() -> Self {
        Self {
            data: [Self::EMPTY; Self::MAX_VAL],
        }
    }
    fn insert(&mut self, index: u32, val: usize) {
        self.data[index as usize] = val;
    }
    fn remove(&mut self, index: u32) -> Option<usize> {
        let val = self.data[index as usize];
        if val == Self::EMPTY {
            None
        } else {
            self.data[index as usize] = Self::EMPTY;
            Some(val)
        }
    }
}

impl Board {
    const LENGTH: usize = 5;
    fn new(id: usize, grid: [u32; 25], set: IntMap) -> Self {
        Self {
            id,
            grid,
            set,
            row_count: [0; Self::LENGTH],
            col_count: [0; Self::LENGTH],
            solved: false,
        }
    }
    fn update(&mut self, draw: u32) -> bool {
        if self.solved {
            return false;
        }
        if let Some(idx) = self.set.remove(draw) {
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
        // self.set.keys().copied().sum()
    }
}

pub fn solve(input: &str) -> Result<Solution<u32, u32>> {
    let first = input.lines().next().unwrap();
    let order: Result<Vec<u32>, _> = first.split(",").map(|s| s.parse()).collect();
    let order = order?;
    debug_assert!({
        // Order has no duplicates
        let mut t = order.clone();
        t.sort();
        t.dedup();
        order.len() == t.len()
    });
    let mut grid: [u32; 25] = [0; 25];
    let mut index = 0;
    // let mut set = HashMap::with_capacity(25);
    let mut set = IntMap::new();
    let mut boards = Vec::new();
    for line in input.lines().skip(2) {
        line.split_whitespace().for_each(|s| {
            let num = s.parse().unwrap();
            grid[index] = num;
            set.insert(num, index);
            index += 1;
        });
        if index >= 25 {
            // debug_assert!(set.len() == 25);
            boards.push(Board::new(boards.len(), grid, set));
            index = 0;
            grid = [0; 25];
            // set = HashMap::with_capacity(25);
            set = IntMap::new();
        }
    }
    let (part1, part2) = run_until_solved(&mut boards, &order);
    Ok(Solution::new(part1, part2))
}

fn run_until_solved(boards: &mut Vec<Board>, order: &[u32]) -> (u32, u32) {
    let mut part1 = 0;
    // let mut remove = Vec::new();
    // for num in order.iter().copied() {
    //     if boards.len() == 1 {
    //         let board = &mut boards[0];
    //         if board.update(num) {
    //             return (part1, num * board.sum_unmarked());
    //         }
    //     } else {
    //         for board in boards.iter_mut() {
    //             if board.update(num) {
    //                 if part1 == 0 {
    //                     part1 = num * board.sum_unmarked();
    //                 }
    //                 remove.push(board.id);
    //             }
    //         }
    //         if remove.len() > 0 {
    //             boards.retain(|b| !remove.contains(&b.id));
    //             remove.clear();
    //         }
    //     }
    // }
    let mut solved = 0;
    let len = boards.len();
    for num in order.iter().copied() {
        for board in boards.iter_mut() {
            if board.update(num) {
                solved += 1;
                if part1 == 0 {
                    part1 = num * board.sum_unmarked() as u32;
                } else if solved == len {
                    return (part1, num * board.sum_unmarked() as u32);
                }
            }
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_intmap() {
        let keys = vec![95, 7, 8, 9, 10, 14, 16, 18, 55, 0];
        let vals: Vec<_> = (0..10).collect();
        let mut map1 = IntMap::new();
        let mut map2 = HashMap::new();
        for (k, v) in keys.into_iter().zip(vals.into_iter()) {
            map1.insert(k, v);
            map2.insert(k, v);
        }
        // let sum2: usize = map2.keys().sum();
        // assert_eq!(sum1, sum2);
    }
}
