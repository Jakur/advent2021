use super::*;
use std::{fmt, ops::Add};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Number {
    Pair,
    Value(u32),
    Empty,
}

// struct LeftIterator<'a> {
//     tree: &'a Tree,
//     idx: usize,
// }

// impl<'a> Iterator for LeftIterator<'a> {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {

//     }
// }

struct Tree {
    data: Vec<Number>,
}

impl Tree {
    fn new() -> Self {
        Self {
            data: vec![Number::Empty; 64],
        }
    }
    fn max_used_index(&self) -> Option<usize> {
        self.data
            .iter()
            .enumerate()
            .rev()
            .find(|(_idx, node)| **node != Number::Empty)
            .map(|(a, _b)| a)
    }
    fn reduce(&mut self) {
        while !self.reduce_inner() {}
    }
    fn reduce_inner(&mut self) -> bool {
        println!("{}", self);
        let order = self.left_vec();
        // [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
        for (order_idx, tree_idx) in order.iter().copied().enumerate() {
            let depth = Self::depth(tree_idx);
            if depth >= 4 && self.data[tree_idx] == Number::Pair {
                self.explode(order_idx, &order);
                return false;
            }
        }
        for tree_idx in order.iter().copied() {
            if let Number::Value(x) = self.data[tree_idx] {
                if x >= 10 {
                    self.split(tree_idx);
                    return false;
                }
            }
        }
        true
    }
    fn split(&mut self, idx: usize) {
        // Todo fix split
        dbg!("Split");
        let left = Self::left_child(idx);
        let right = Self::right_child(idx);
        debug_assert!(self.data[left] == Number::Empty);
        debug_assert!(self.data[right] == Number::Empty);
        match self.data[idx] {
            Number::Value(x) => {
                let left_val = x / 2;
                let right_val = if x % 2 == 0 { x / 2 } else { x / 2 + 1 };
                self.data[left] = Number::Value(left_val);
                self.data[right] = Number::Value(right_val);
            }
            _ => unimplemented!(),
        }
    }
    fn explode(&mut self, order_idx: usize, order: &[usize]) {
        dbg!("Explode");
        let start = order[order_idx];
        let left_idx = Self::left_child(start);
        let right_idx = Self::right_child(start);
        let left_val = match self.data[left_idx] {
            Number::Value(x) => x,
            Number::Empty => {
                dbg!(self.data[left_idx]);
                unimplemented!()
            }
            Number::Pair => {
                println!("Too Deep");
                unimplemented!()
            }
        };
        let right_val = match self.data[right_idx] {
            Number::Value(x) => x,
            _ => unimplemented!(),
        };
        self.data[left_idx] = Number::Empty;
        self.data[right_idx] = Number::Empty;
        self.data[start] = Number::Value(0);
        for idx in order[0..order_idx].iter().rev() {
            if let Number::Value(x) = self.data[*idx] {
                self.data[*idx] = Number::Value(x + left_val);
                break;
            }
        }
        for idx in order[order_idx + 1..].iter() {
            if let Number::Value(x) = self.data[*idx] {
                self.data[*idx] = Number::Value(x + right_val);
                break;
            }
        }
    }
    fn left_vec(&self) -> Vec<usize> {
        let mut vec = Vec::with_capacity(64);
        self.left_recurse(&mut vec, 1);
        vec
    }
    fn left_recurse(&self, vec: &mut Vec<usize>, idx: usize) {
        if idx >= self.data.len() {
            return;
        }
        if let Number::Empty = self.data[idx] {
            return;
        }
        let left = Self::left_child(idx);
        self.left_recurse(vec, left);

        vec.push(idx);

        let right = Self::right_child(idx);
        self.left_recurse(vec, right);
    }
    const fn left_child(idx: usize) -> usize {
        idx * 2
    }
    const fn right_child(idx: usize) -> usize {
        idx * 2 + 1
    }
    const fn depth(idx: usize) -> u32 {
        usize::BITS - 1 - usize::leading_zeros(idx)
    }
}

impl Add for Tree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(self.max_used_index().unwrap() <= 31 && self.max_used_index().unwrap() <= 31);
        let mut data = vec![Number::Empty, Number::Pair];
        let mut left = self.data.into_iter().skip(1);
        let mut right = rhs.data.into_iter().skip(1);
        let mut take = 1;
        while data.len() < 64 {
            for _ in 0..take {
                data.push(left.next().unwrap());
            }
            for _ in 0..take {
                data.push(right.next().unwrap());
            }
            take *= 2;
        }
        let mut out_tree = Tree { data };
        out_tree.reduce();
        out_tree
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        construct_line(self, 1, &mut string);
        write!(f, "{}", string)
    }
}

fn construct_line(tree: &Tree, idx: usize, string: &mut String) {
    match tree.data[idx] {
        Number::Pair => {
            string.push_str("[");
            construct_line(tree, idx * 2, string);
            string.push_str(",");
            construct_line(tree, idx * 2 + 1, string);
            string.push_str("]");
        }
        Number::Value(x) => string.push_str(&format!("{}", x)),
        Number::Empty => {}
    }
}

fn parse_line(line: &str) -> Tree {
    let mut idx = 1;
    let mut tree = Tree::new();
    for c in line.chars() {
        match c {
            '[' => idx *= 2,
            ']' => idx /= 2,
            ',' => idx += 1,
            _ => {
                let node = Number::Value(c.to_digit(10).unwrap());
                tree.data[idx] = node;
            }
        }
    }
    for idx in (0..tree.data.len()).rev() {
        match tree.data[idx] {
            Number::Pair | Number::Value(_) => {
                tree.data[idx / 2] = Number::Pair;
            }
            Number::Empty => {}
        }
    }
    tree
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    let trees: Vec<_> = input.lines().map(|line| parse_line(line)).collect();
    for (tree, line) in trees.iter().zip(input.lines()) {
        // Test my parsing works
        assert_eq!(&format!("{}", tree), line);
    }

    let deepest_node = trees.iter().map(|t| t.max_used_index()).max();
    dbg!(deepest_node);

    let t1 = parse_line("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let t2 = parse_line("[1,1]");
    let t3 = t1 + t2;
    println!("Finished: {}", t3);
    // let s = "[[1,9],[8,5]]";
    // let tree = parse_line(s);
    // println!("{}", tree);
    Solution::new(0, 0)
}
