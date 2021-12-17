use super::*;
use std::collections::HashMap;

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
    fn check_point(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }
    fn check_x(&self, x: i32) -> bool {
        self.min_x <= x && x <= self.max_x
    }
    fn check_y(&self, y: i32) -> bool {
        self.min_y <= y && y <= self.max_y
    }
    /// Checks the minimum and maximum steps where the path will fall within
    /// the target. If the path never falls within the target, return None
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
        let mut hit = false;
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
            // hit = hit || self.check_y(y);
        }
        // hit = hit || self.check_y(y);
        ret
    }
}

struct StepsMap {
    single_counts: HashMap<usize, usize>,
    geq: Vec<usize>,
}

impl StepsMap {
    fn new(single_counts: HashMap<usize, usize>, geq: Vec<usize>) -> Self {
        Self { single_counts, geq }
    }
    fn get(&self, steps: usize) -> usize {
        let single = self.single_counts.get(&steps).copied().unwrap_or_default();
        self.geq.iter().filter(|&&c| steps >= c).count() + single
    }
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    // Idea: x and y are independent, so let us first find all the legal step
    // counts that we can have that get us to the proper x-range
    // target area: x=20..30, y=-10..-5
    // let target = Target::new(139, 187, -148, -89);
    let target = Target::new(20, 30, -10, -5);
    let mut steps = Vec::new();
    let mut geq = Vec::new();
    for x_velocity in (0..200).rev() {
        // println!("{}", x_velocity);
        match target.check_x_path(x_velocity) {
            StepsX::Never => {}
            StepsX::Range(a, b) => {
                if a != b {
                    println!("a: {}, b: {}", a, b);
                }
                for x in a..=b {
                    steps.push(x);
                }
            }
            StepsX::GreaterEq(x) => {
                println!("With x_velocity: {} Steps >= {}", x_velocity, x);
                geq.push(x);
            }
        }
    }
    // let mut steps: Vec<_> = steps.into_iter().collect();
    steps.sort();
    let mut map = HashMap::new();
    for s in steps.clone() {
        *map.entry(s).or_default() += 1;
    }
    let steps_map = StepsMap::new(map, geq);
    for s in steps {
        println!("Steps: {}", s);
    }
    println!("{}", steps_map.get(17));
    let mut count = 0;
    // Idk why 10_000 is sufficient to be honest
    for y_velocity in -150..10000 {
        if let Some(legal) = target.check_y_path(y_velocity).valid() {
            let old = count;
            if legal.min != legal.max {
                println!("LEGAL: {} {}", legal.min, legal.max);
            }
            for steps in legal.min..=legal.max {
                count += steps_map.get(steps);
            }
            // count += 1;
            println!("y: {}, x_count?: {}", y_velocity, count - old);
            // dbg!(max_value);
        }
    }
    println!("{}", count);
    Solution::new(0, 0)
}
