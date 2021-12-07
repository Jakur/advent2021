use super::*;

pub fn solve(input: &[u8]) -> Result<Solution<u64, u64>> {
    let mut fish_count: [u64; 9] = [0; 9];
    let mut idx = 0;
    while idx < input.len() {
        let num = input[idx] - b'0';
        fish_count[num as usize] += 1;
        idx += 2;
    }
    for _ in 0..80 {
        update_step(&mut fish_count);
    }
    let part1 = fish_count.iter().copied().sum();
    for _ in 0..256 - 80 {
        update_step(&mut fish_count);
    }
    let part2 = fish_count.iter().copied().sum();
    Ok(Solution::new(part1, part2))
}

fn update_step(fish_count: &mut [u64; 9]) {
    let zero = fish_count[0];
    for idx in 1..9 {
        let val = fish_count[idx];
        fish_count[idx - 1] += val;
        fish_count[idx] -= val;
    }
    fish_count[0] -= zero;
    fish_count[6] += zero;
    fish_count[8] += zero;
}
