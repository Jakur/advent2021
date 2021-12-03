use super::*;
use anyhow::anyhow;

#[derive(Debug)]
struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

pub fn solve(input: &str) -> Result<Solution<u32, u32>> {
    let mut pos = Position {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };
    for line in input.lines() {
        // Each line has at most one digit
        let num = line
            .chars()
            .rev()
            .next()
            .and_then(|c| c.to_digit(10))
            .ok_or_else(|| anyhow!("Expected to parse a digit"))?;
        if line.starts_with("down") {
            pos.aim += num;
        } else if line.starts_with("up") {
            pos.aim -= num;
        } else {
            pos.horizontal += num;
            pos.depth += pos.aim * num;
        }
    }
    Ok(Solution::new(
        pos.aim * pos.horizontal,
        pos.depth * pos.horizontal,
    ))
}
