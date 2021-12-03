use super::*;
use anyhow::ensure;

enum MostCommon {
    Zero,
    One,
    Tie,
}

pub fn solve(input: &str) -> Result<Solution<u32, u32>> {
    const NUM_BITS: usize = 12;
    const CHECK: u32 = 1 << (NUM_BITS - 1);
    let mut set_bit_count: [u32; NUM_BITS] = [0; NUM_BITS];
    let mut ones = Vec::with_capacity(1200);
    let mut zeros = Vec::with_capacity(1200);
    for line in input.lines() {
        let mut num = u32::from_str_radix(line, 2)?;
        // Perform the first split for part 2 to avoid extra vector allocations
        if num >= CHECK {
            ones.push(num);
        } else {
            zeros.push(num);
        }
        for bit in 0..set_bit_count.len() {
            set_bit_count[bit] += num & 1;
            num = num >> 1;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let count = (ones.len() + zeros.len()) as u32;
    for (i, set_count) in set_bit_count.iter().enumerate() {
        // 1 is the most common bit
        if *set_count > count / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }
    let (mut oxy, mut co2) = if set_bit_count[NUM_BITS - 1] > count / 2 {
        // 1 is the most common bit in the highest slot
        (ones, zeros)
    } else {
        (zeros, ones)
    };
    for shift in (0..NUM_BITS - 1).rev() {
        let test_num = 1 << shift;
        let retain = match most_common_outcome(&oxy, test_num) {
            MostCommon::One | MostCommon::Tie => test_num,
            MostCommon::Zero => 0,
        };
        oxy.retain(|&x| x & test_num == retain);
        if oxy.len() == 1 {
            break;
        }
    }
    for shift in (0..NUM_BITS - 1).rev() {
        let test_num = 1 << shift;
        let remove = match most_common_outcome(&co2, test_num) {
            MostCommon::One | MostCommon::Tie => test_num,
            MostCommon::Zero => 0,
        };
        co2.retain(|&x| x & test_num != remove);
        if co2.len() == 1 {
            break;
        }
    }
    ensure![oxy.len() == 1, "Oxygen reading did not converge properly"];
    ensure![co2.len() == 1, "Co2 reading did not converge properly"];
    Ok(Solution::new(gamma * epsilon, oxy[0] * co2[0]))
}

fn most_common_outcome(slice: &[u32], test: u32) -> MostCommon {
    let mut zeros = 0;
    let mut ones = 0;
    for x in slice {
        if x & test == 0 {
            zeros += 1;
        } else {
            ones += 1;
        }
    }
    if zeros > ones {
        MostCommon::Zero
    } else if ones > zeros {
        MostCommon::One
    } else {
        MostCommon::Tie
    }
}
