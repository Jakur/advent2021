use super::*;
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

#[derive(Clone)]
struct Visit {
    current_node: usize,
    history: u32,
    long_visits: bool, // Long visits remaining
}

impl Visit {
    fn new(current_node: usize, mut history: u32, mut long_visits: bool) -> Self {
        let check = 1 << current_node;
        if long_visits && current_node >= (END + 1) && (check & history == check) {
            long_visits = false;
        }
        history |= check;
        Self {
            current_node,
            history,
            long_visits,
        }
    }
    fn index(&self, index: usize) -> bool {
        let shifted = 1 << index;
        (shifted & self.history) == shifted
    }
}

struct Graph<'a> {
    upper: &'a HashMap<&'a str, usize>,
    lower: &'a HashMap<&'a str, usize>,
    length: usize,
    adj_matrix: Vec<bool>,
}

impl<'a> Graph<'a> {
    pub fn new(upper: &'a HashMap<&'a str, usize>, lower: &'a HashMap<&'a str, usize>) -> Self {
        let length = upper.len() + lower.len();
        let adj_matrix = vec![false; length * length];
        Self {
            upper,
            lower,
            adj_matrix,
            length,
        }
    }
    pub fn get_index(&self, node: &str) -> usize {
        if let Some(num) = self.lower.get(node) {
            *num
        } else {
            let num = self.upper.get(node).unwrap();
            num + self.lower.len()
        }
    }
    pub fn set_edge(&mut self, node1: &str, node2: &str) {
        let idx1 = self.get_index(node1);
        let idx2 = self.get_index(node2);
        self.adj_matrix[idx1 * self.length + idx2] = true;
        self.adj_matrix[idx2 * self.length + idx1] = true;
    }
    pub fn part1(&self) -> usize {
        let visit_vec = 1; // Cannot revisit the first point
        let stack: Vec<Visit> = vec![Visit::new(0, visit_vec, false)];
        self.explore(stack)
    }
    pub fn part2(&self) -> usize {
        let visit_vec = 1; // Cannot revisit the first point
        let stack: Vec<Visit> = vec![Visit::new(0, visit_vec, true)];
        self.explore(stack)
    }
    pub fn explore(&self, mut stack: Vec<Visit>) -> usize {
        let mut count = 0;
        while let Some(val) = stack.pop() {
            if val.current_node == END {
                count += 1;
                continue;
            }
            for idx in 1..self.lower.len() {
                if self.adj_matrix[self.length * val.current_node + idx]
                    && (val.long_visits || !val.index(idx))
                {
                    stack.push(Visit::new(idx, val.history.clone(), val.long_visits))
                }
            }
            for idx in self.lower.len()..self.length {
                if self.adj_matrix[self.length * val.current_node + idx] {
                    let mut next_visit = val.clone();
                    next_visit.current_node = idx;
                    stack.push(next_visit)
                }
            }
        }
        count
    }
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    let mut upper = HashMap::new();
    let mut lower = HashMap::new();
    lower.insert("start", START);
    lower.insert("end", END);
    for line in input.lines() {
        let mut sp = line.split("-");
        let n1 = sp.next().unwrap();
        let n2 = sp.next().unwrap();
        if is_uppercase(n1) {
            str_map_insert(n1, &mut upper);
        } else {
            str_map_insert(n1, &mut lower);
        }
        if is_uppercase(n2) {
            str_map_insert(n2, &mut upper);
        } else {
            str_map_insert(n2, &mut lower);
        }
    }
    let mut graph = Graph::new(&upper, &lower);
    for line in input.lines() {
        let mut sp = line.split("-");
        let n1 = sp.next().unwrap();
        let n2 = sp.next().unwrap();
        graph.set_edge(n1, n2);
    }
    Solution::new(graph.part1() as u32, graph.part2() as u32)
}

fn str_map_insert<'a>(s: &'a str, map: &mut HashMap<&'a str, usize>) {
    let len = map.len();
    map.entry(s).or_insert(len);
}

fn is_uppercase(s: &str) -> bool {
    let c = s.chars().next().unwrap();
    c.to_uppercase().next() == Some(c)
}
