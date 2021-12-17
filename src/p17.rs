use super::*;
use std::collections::{HashMap, HashSet};

enum StepsX {
    Never,
    Range(usize, usize),
    GreaterEq(usize),
}

struct StepsY {
    min: usize,
    max: usize,
    height: i32,
}

impl StepsY {
    fn update(&mut self, steps: usize) {
        if self.min == 0 {
            self.min = steps;
        }
        self.max = steps;
    }
    fn new() -> Self {
        Self {
            min: 0,
            max: 0,
            height: 0,
        }
    }
    fn height(mut self, height: i32) -> Self {
        self.height = height;
        self
    }
    fn valid(self) -> Option<Self> {
        if self.min > 0 {
            Some(self)
        } else {
            None
        }
    }
}

struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Target {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
    fn check_x(&self, x: i32) -> bool {
        self.min_x <= x && x <= self.max_x
    }
    fn check_y(&self, y: i32) -> bool {
        self.min_y <= y && y <= self.max_y
    }
    fn check_x_path(&self, mut velocity_x: i32) -> StepsX {
        let mut x = 0;
        let mut step_count = 0;
        let mut first_within = None;
        let mut last_within = 0;
        while velocity_x > 0 && x <= self.max_x {
            x += velocity_x;
            velocity_x -= 1;
            step_count += 1;
            if self.check_x(x) {
                if first_within.is_none() {
                    first_within = Some(step_count);
                }
                last_within = step_count;
            }
        }
        if velocity_x == 0 {
            if x < self.min_x {
                return StepsX::Never;
            } else if self.check_x(x) {
                return StepsX::GreaterEq(first_within.unwrap());
            }
        }
        if let Some(first) = first_within {
            StepsX::Range(first, last_within)
        } else {
            StepsX::Never // Completely overshot it
        }
    }

    fn check_y_path(&self, mut velocity_y: i32) -> StepsY {
        let mut y = 0;
        let mut ret = StepsY::new();
        let mut steps = 0;
        while y >= self.min_y {
            if velocity_y == 0 {
                ret = ret.height(y);
            }
            steps += 1;
            y += velocity_y;
            velocity_y -= 1;
            if self.check_y(y) {
                ret.update(steps);
            }
        }
        ret
    }
}

struct StepsMap {
    single_counts: HashMap<usize, Vec<i32>>,
    geq: Vec<(usize, i32)>,
}

impl StepsMap {
    fn new(single_counts: HashMap<usize, Vec<i32>>, geq: Vec<(usize, i32)>) -> Self {
        Self { single_counts, geq }
    }
    fn get(&self, steps: usize) -> Vec<i32> {
        let mut single = self
            .single_counts
            .get(&steps)
            .map(|x| x.clone())
            .unwrap_or_else(|| Vec::new());
        single.extend(
            self.geq
                .iter()
                .filter_map(|&c| if steps >= c.0 { Some(c.1) } else { None }),
        );
        single
    }
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    // Idea: x and y are independent, so let us first find all the legal step
    // counts that we can have that get us to the proper x-range
    let split: Vec<i32> = input
        .split(|c| match c {
            '=' | '.' | ',' | '\n' => true,
            _ => false,
        })
        .filter_map(|x| x.parse().ok())
        .collect();
    let target = Target::new(split[0], split[1], split[2], split[3]);
    // let target = Target::new(20, 30, -10, -5);
    let mut steps = Vec::new();
    let mut geq = Vec::new();
    for x_velocity in (0..200).rev() {
        match target.check_x_path(x_velocity) {
            StepsX::Never => {}
            StepsX::Range(a, b) => {
                for x in a..=b {
                    steps.push((x, x_velocity));
                }
            }
            StepsX::GreaterEq(x) => {
                geq.push((x, x_velocity));
            }
        }
    }
    let mut map: HashMap<usize, Vec<i32>> = HashMap::new();
    for (s, x) in steps.clone() {
        let entry = map.entry(s).or_default();
        entry.push(x);
    }
    let beyond = geq[0].0;
    let steps_map = StepsMap::new(map, geq);

    // Check that every positive number of steps has an initial x velocity ending in the grid
    debug_assert!((1..beyond).all(|x| steps_map.single_counts.get(&x).is_some()));

    let mut count = 0;
    let mut y_max = 0;
    // The velocity at the origin should be -1 * initial_velocity
    for y_velocity in -150..150 {
        if let Some(legal) = target.check_y_path(y_velocity).valid() {
            y_max = std::cmp::max(legal.height, y_max);
            if legal.min == legal.max {
                let set = steps_map.get(legal.min);
                count += set.len();
            } else {
                let mut set = HashSet::new();
                for steps in legal.min..=legal.max {
                    set.extend(steps_map.get(steps).into_iter());
                }
                count += set.len();
            }
        }
    }
    Solution::new(y_max as u32, count as u32)
}
