use super::*;

pub fn solve(input: &str) -> Result<Solution<i32, i32>> {
    let vec: Result<Vec<i32>, _> = input.trim().split(",").map(|x| x.parse()).collect();
    let vec = vec?;
    let max = *vec.iter().max().unwrap();
    let min = *vec.iter().min().unwrap();
    let mut min_fuel_idx = 0;
    let mut min_fuel_idx2 = 0;
    let mut min_fuel = i32::MAX;
    let mut min_fuel2 = i32::MAX;
    for goal in min..=max {
        let total = cost(goal, &vec);
        let total2 = cost2(goal, &vec);
        if total < min_fuel {
            min_fuel_idx = goal;
            min_fuel = total;
        }
        if total2 < min_fuel2 {
            min_fuel_idx2 = goal;
            min_fuel2 = total2;
        }
    }
    dbg!(min_fuel_idx);
    dbg!(min_fuel_idx2);
    // dbg!(seq);
    Ok(Solution::new(min_fuel, min_fuel2))
}

fn cost(goal: i32, slice: &[i32]) -> i32 {
    slice.iter().copied().map(|x| (goal - x).abs()).sum()
}

fn cost2(goal: i32, slice: &[i32]) -> i32 {
    // n*(n+1)/2
    slice
        .iter()
        .copied()
        .map(|x| {
            let val = (goal - x).abs();
            (val * (val + 1)) / 2
        })
        .sum()
}
