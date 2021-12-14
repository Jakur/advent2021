use std::collections::HashMap;

use super::*;

type CMap = HashMap<(u8, u8), u64>;

pub fn solve(input: &str) -> Solution<u64, u64> {
    let first = input.lines().next().unwrap();
    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        let mut sp = line.split(" -> ");
        let k = sp.next().unwrap().as_bytes();
        debug_assert_eq!(k.len(), 2);
        let v = sp.next().unwrap().as_bytes();
        debug_assert_eq!(v.len(), 1);
        rules.insert((k[0], k[1]), v[0]);
    }

    let vec: Vec<_> = first.as_bytes().into();
    let mut counts: HashMap<_, u64> = HashMap::new();
    for (a, b) in vec.iter().zip(vec[1..].iter()) {
        *counts.entry((*a, *b)).or_default() += 1;
    }

    let first_c = vec[0];
    let last_c = vec[vec.len() - 1];
    counts = run_iterations(counts, &rules, 10);
    let part1 = count_score(first_c, last_c, &counts);
    counts = run_iterations(counts, &rules, 40 - 10);
    let part2 = count_score(first_c, last_c, &counts);
    Solution::new(part1, part2)
}

fn count_score(first: u8, last: u8, counts: &CMap) -> u64 {
    let mut char_counts: Vec<u64> = vec![0; 255];
    // We store the counts of every substring of size 2
    // Thus every single character is double counted except the first and last character
    // We don't know the order of the final sequence except, luckily, the first and last
    // Because characters are only generated in the middle, we still have the initial
    // First and last characters. So we increment them by 1, and then we can count
    char_counts[first as usize] += 1;
    char_counts[last as usize] += 1;
    for (k, v) in counts.iter() {
        char_counts[k.0 as usize] += *v;
        char_counts[k.1 as usize] += *v;
    }
    let min = char_counts
        .iter()
        .copied()
        .filter(|&x| x > 0)
        .min()
        .unwrap()
        / 2;
    let max = char_counts.iter().copied().max().unwrap() / 2;
    max - min
}

fn run_iterations(mut counts: CMap, rules: &HashMap<(u8, u8), u8>, num_iter: usize) -> CMap {
    for _iter in 0..num_iter {
        let mut update: HashMap<_, u64> = HashMap::new();
        for (k, v) in counts.iter() {
            let created = *rules.get(k).unwrap();
            let n1 = (k.0, created);
            let n2 = (created, k.1);
            *update.entry(n1).or_default() += v;
            *update.entry(n2).or_default() += v;
        }
        counts = update;
    }
    counts
}
