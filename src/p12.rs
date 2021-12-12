use super::*;
use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

struct Visit {
    current_node: usize,
    history: Vec<u8>,
    free_long_visit: bool,
}

impl Visit {
    fn new(current_node: usize, mut history: Vec<u8>, mut free_long_visit: bool) -> Self {
        if current_node < history.len() {
            history[current_node] += 1;
            if history[current_node] == 2 && current_node >= END + 1 {
                free_long_visit = false;
            }
        }
        Self {
            current_node,
            history,
            free_long_visit,
        }
    }
    fn max_visit_len(&self) -> u8 {
        self.free_long_visit as u8 + 1
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
    pub fn is_upper(&self, node: usize) -> bool {
        node >= self.lower.len()
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
    pub fn explore(&self) -> usize {
        let mut visit_vec = vec![0u8; self.lower.len()];
        visit_vec[0] += 1; // Cannot revisit the first point;
        let mut stack: Vec<Visit> = vec![Visit::new(0, visit_vec, false)];
        let mut count = 0;
        while let Some(val) = stack.pop() {
            if val.current_node == END {
                count += 1;
                continue;
            }
            for idx in 0..self.length {
                if self.adj_matrix[self.length * val.current_node + idx]
                    && *val.history.get(idx).unwrap_or(&0) == 0
                {
                    stack.push(Visit::new(idx, val.history.clone(), false))
                }
            }
        }
        count
    }
    pub fn explore2(&self) -> usize {
        let mut visit_vec = vec![0u8; self.lower.len()];
        visit_vec[0] += 1; // Cannot revisit the first point;
        let mut stack: Vec<Visit> = vec![Visit::new(0, visit_vec, true)];
        let mut count = 0;
        while let Some(val) = stack.pop() {
            if val.current_node == END {
                count += 1;
                continue;
            }
            for idx in 0..self.length {
                if self.adj_matrix[self.length * val.current_node + idx]
                    && (*val.history.get(idx).unwrap_or(&0)) < val.max_visit_len()
                {
                    stack.push(Visit::new(idx, val.history.clone(), val.free_long_visit))
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
    // dbg!(graph.adj_matrix.iter().filter(|&&x| x == true).count());
    Solution::new(graph.explore() as u32, graph.explore2() as u32)
}

fn str_map_insert<'a>(s: &'a str, map: &mut HashMap<&'a str, usize>) {
    let len = map.len();
    map.entry(s).or_insert(len);
}

fn is_uppercase(s: &str) -> bool {
    let c = s.chars().next().unwrap();
    c.to_uppercase().next() == Some(c)
}
