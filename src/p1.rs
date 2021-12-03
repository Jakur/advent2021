use crate::Solution;

pub fn solve(input: &[u8]) -> Solution<u32, u32> {
    let mut vec: Vec<u32> = Vec::with_capacity(2000);
    let mut num = 0;
    for byte in input.iter().copied() {
        if byte == b'\n' {
            vec.push(num);
            num = 0;
        } else {
            num = num * 10 + (byte & 0x0f) as u32;
        }
    }
    let mut count1 = 0;
    for window in vec.windows(2) {
        if window[0] < window[1] {
            count1 += 1;
        }
    }
    let mut count2 = 0;
    // [0, 1, 2, 3]
    // Window 1: [0, 1, 2]
    // Window 2: [1, 2, 3]
    // Both contain [1, 2] so we can just ignore them and compare the head and tail.
    // In theory this would fail as we approach the end of the list, however
    // All numbers in the input are non-negative, so if the rightmost slice has
    // fewer numbers it cannot be greater than the previous slice, so it's fine
    // to not consider it
    for window in vec.windows(4) {
        if window[0] < window[3] {
            count2 += 1;
        }
    }
    Solution::new(count1, count2)
}
