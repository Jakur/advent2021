use super::*;
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};

const INIT_ROWS: usize = 100;
const INIT_COLS: usize = 100;
const ROWS: usize = 5 * INIT_ROWS;
const COLS: usize = 5 * INIT_COLS;

pub fn solve(input: &[u8]) -> Solution<u32, u32> {
    // Let's just be straightforward here and use a library
    let mut graph = DiGraph::new();
    for _ in 0..ROWS * COLS {
        graph.add_node(());
    }
    for row in 0..ROWS {
        let row_board_off = (row / INIT_ROWS) as u32;
        let row_mod = row % INIT_ROWS;
        for col in 0..COLS {
            let col_board_off = (col / INIT_COLS) as u32;
            let col_mod = col % INIT_COLS;
            let weight = (input[row_mod * (INIT_COLS + 1) + col_mod] - b'0') as u32;
            let mut weight = weight + row_board_off + col_board_off;
            if weight > 9 {
                weight = weight % 10 + 1;
            }
            let from = &[
                west(row, col),
                east(row, col),
                north(row, col),
                south(row, col),
            ];
            for idx in from {
                if let Some(node) = idx {
                    graph.add_edge(
                        NodeIndex::new(*node),
                        NodeIndex::new(row * COLS + col),
                        weight,
                    );
                }
            }
        }
    }

    let idx1 = NodeIndex::new(INIT_ROWS * COLS - (COLS - INIT_COLS) - 1);
    let idx2 = NodeIndex::new(ROWS * COLS - 1);
    let res = dijkstra(&graph, NodeIndex::new(0), Some(idx2), |x| *x.weight());
    let part1 = *res.get(&idx1).unwrap();
    let part2 = *res.get(&idx2).unwrap();
    Solution::new(part1, part2)
}

pub fn west(row: usize, col: usize) -> Option<usize> {
    Some(row * COLS + col.checked_sub(1)?)
}
pub fn east(row: usize, col: usize) -> Option<usize> {
    if col < COLS - 1 {
        Some(row * COLS + col + 1)
    } else {
        None
    }
}
pub fn north(row: usize, col: usize) -> Option<usize> {
    Some(row.checked_sub(1)? * COLS + col)
}
pub fn south(row: usize, col: usize) -> Option<usize> {
    if row < ROWS - 1 {
        Some((row + 1) * COLS + col)
    } else {
        None
    }
}
