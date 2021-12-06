use super::*;

pub fn solve(input: &str) -> Result<Solution<u64, u64>> {
    let mut fish_count: Vec<u64> = vec![0; 9];
    for x in input.trim().split(",") {
        let val = x.parse::<usize>()?;
        fish_count[val] += 1;
    }
    dbg!(&fish_count);
    for _ in 0..80 {
        fish_count = update_step(fish_count);
    }
    let part1 = fish_count.iter().copied().sum();
    for _ in 0..256 - 80 {
        fish_count = update_step(fish_count);
    }
    let part2 = fish_count.iter().copied().sum();
    Ok(Solution::new(part1, part2))
}

fn update_step(fish_count: Vec<u64>) -> Vec<u64> {
    let mut update = fish_count.clone();
    for (idx, val) in fish_count.into_iter().enumerate() {
        if idx == 0 {
            update[6] += val;
            update[8] += val;
        } else {
            update[idx - 1] += val;
        }
        update[idx] -= val;
    }
    update
}
