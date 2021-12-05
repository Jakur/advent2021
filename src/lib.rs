pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
use anyhow::Result;
use std::fmt::Display;

pub static DAY1: &[u8] = include_bytes!("../input/day1");
pub static DAY2: &[u8] = include_bytes!("../input/day2");
pub static DAY3: &[u8] = include_bytes!("../input/day3");
pub static DAY4: &[u8] = include_bytes!("../input/day4");
pub static DAY5: &'static str = include_str!("../input/day5");

#[derive(Debug)]
pub struct Solution<A, B> {
    part1: A,
    part2: B,
}

impl<A, B> Solution<A, B> {
    pub fn new(part1: A, part2: B) -> Self {
        Self { part1, part2 }
    }
}

impl<A, B> Display for Solution<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Part 1: {}\nPart 2: {}", self.part1, self.part2)
    }
}
