use super::*;
use std::collections::HashMap;

type Map = HashMap<Position, u32>;
// type UpdateFn = Fn(Position) -> Position;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn north(self) -> Self {
        Self::new(self.x, self.y + 1)
    }
    fn east(self) -> Self {
        Self::new(self.x + 1, self.y)
    }
    fn south(self) -> Self {
        Self::new(self.x, self.y - 1)
    }
    fn west(self) -> Self {
        Self::new(self.x - 1, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}

fn update<F>(mut start: Position, end: Position, map: &mut Map, func: F)
where
    F: Fn(Position) -> Position,
{
    while start != end {
        *map.entry(start).or_default() += 1;
        start = func(start);
    }
    *map.entry(end).or_default() += 1;
}

pub fn solve(input: &'static str) -> Result<Solution<u32, u32>> {
    let mut marks: HashMap<Position, u32> = HashMap::new();
    let mut defer = Vec::new();
    for line in input.lines() {
        let mut vals: Vec<u32> = Vec::new();
        for s in line.split("->") {
            let nums = s.split(",");
            for num in nums {
                vals.push(num.trim().parse()?);
            }
        }
        let x1 = vals[0];
        let y1 = vals[1];
        let x2 = vals[2];
        let y2 = vals[3];
        let start = Position::new(x1, y1);
        let end = Position::new(x2, y2);
        debug_assert!(!(x1 == x2 && y1 == y2));
        let dir = {
            if x1 == x2 {
                if y2 > y1 {
                    Direction::North
                } else {
                    Direction::South
                }
            } else if y1 == y2 {
                if x2 > x1 {
                    Direction::East
                } else {
                    Direction::West
                }
            } else {
                if x2 > x1 {
                    if y2 > y1 {
                        Direction::Northeast
                    } else {
                        Direction::Southeast
                    }
                } else {
                    if y2 > y1 {
                        Direction::Northwest
                    } else {
                        Direction::Southwest
                    }
                }
            }
        };

        let func = match dir {
            Direction::North => Position::north,
            Direction::East => Position::east,
            Direction::South => Position::south,
            Direction::West => Position::west,
            _ => {
                defer.push((start, end, dir));
                continue;
            }
        };
        update(start, end, &mut marks, func);
    }
    let part1 = marks.values().filter(|&&x| x >= 2).count();
    for (start, end, dir) in defer.into_iter() {
        let func = match dir {
            Direction::Northeast => |x: Position| x.north().east(),
            Direction::Northwest => |x: Position| x.north().west(),
            Direction::Southeast => |x: Position| x.south().east(),
            Direction::Southwest => |x: Position| x.south().west(),
            _ => unreachable!(),
        };
        update(start, end, &mut marks, func);
    }
    let part2 = marks.values().filter(|&&x| x >= 2).count();
    Ok(Solution::new(part1 as u32, part2 as u32))
}
