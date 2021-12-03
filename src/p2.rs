use super::*;

#[derive(Debug)]
struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

pub fn solve(input: &[u8]) -> Solution<u32, u32> {
    let mut pos = Position {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'd' => {
                i += 7;
                let num = (input[i - 2] - b'0') as u32;
                pos.aim += num;
            }
            b'u' => {
                i += 5;
                let num = (input[i - 2] - b'0') as u32;
                pos.aim -= num;
            }
            _ => {
                debug_assert_eq!(b'f', input[i]);
                i += 10;
                let num = (input[i - 2] - b'0') as u32;
                pos.horizontal += num;
                pos.depth += pos.aim * num;
            }
        }
    }
    Solution::new(pos.aim * pos.horizontal, pos.depth * pos.horizontal)
}
