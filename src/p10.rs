use super::*;

#[derive(Clone, Copy, Debug)]
enum Bracket {
    OpenParen,
    CloseParen,
    OpenDiamond,
    CloseDiamond,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

impl Bracket {
    fn open(self) -> bool {
        use Bracket::*;
        match self {
            OpenParen | OpenDiamond | OpenSquare | OpenCurly => true,
            _ => false,
        }
    }
    fn score(self) -> u32 {
        use Bracket::*;
        match self {
            OpenParen | CloseParen => 3,
            OpenSquare | CloseSquare => 57,
            OpenCurly | CloseCurly => 1197,
            OpenDiamond | CloseDiamond => 25137,
        }
    }
    fn incomplete_score(self) -> u64 {
        use Bracket::*;
        match self {
            OpenParen => 1,
            OpenSquare => 2,
            OpenCurly => 3,
            OpenDiamond => 4,
            _ => unimplemented!(),
        }
    }
}

pub fn solve(input: &[u8]) -> Solution<u32, u64> {
    let mut stack: Vec<Bracket> = Vec::new();
    let mut idx = 0;
    let mut part1 = 0;
    let mut incomplete_scores = Vec::new();
    while idx < input.len() {
        let bracket = match input[idx] {
            b'(' => Bracket::OpenParen,
            b')' => Bracket::CloseParen,
            b'<' => Bracket::OpenDiamond,
            b'>' => Bracket::CloseDiamond,
            b'[' => Bracket::OpenSquare,
            b']' => Bracket::CloseSquare,
            b'{' => Bracket::OpenCurly,
            b'}' => Bracket::CloseCurly,
            b'\n' => {
                let mut score = 0;
                while let Some(inc) = stack.pop() {
                    score *= 5;
                    score += inc.incomplete_score();
                }
                incomplete_scores.push(score);
                idx += 1;
                continue;
            }
            _ => unimplemented!(),
        };
        if bracket.open() {
            stack.push(bracket);
        } else {
            let score = bracket.score();
            if score != stack.pop().unwrap().score() {
                // Invalid Line, clear stack
                stack.clear();
                part1 += score;
                while input[idx] != b'\n' {
                    idx += 1;
                }
            }
        }
        idx += 1;
    }
    incomplete_scores.sort();
    Solution::new(part1, incomplete_scores[incomplete_scores.len() / 2])
}
