use super::*;

struct SegmentDecoder {
    data: [u8; 7],
}

impl SegmentDecoder {
    fn new() -> Self {
        Self { data: [0; 7] }
    }
    fn set(&mut self, encoded: u8, decoded: u8) {
        self.data[encoded as usize] = 1 << (6 - decoded);
    }
    fn decode(&self, values: &[u8]) -> u32 {
        let mut display = 0u8;
        for x in values {
            display |= self.data[(x - b'a') as usize];
        }
        match display {
            0b01110111 => 0,
            0b00010010 => 1,
            0b01011101 => 2,
            0b01011011 => 3,
            0b00111010 => 4,
            0b01101011 => 5,
            0b01101111 => 6,
            0b01010010 => 7,
            0b01111111 => 8,
            0b01111011 => 9,
            _ => unimplemented!(),
        }
    }
}

pub fn solve(input: &'static str) -> Result<Solution<u32, u32>> {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let mut decoder = SegmentDecoder::new();
        let mut split = line.split(" | ");
        let first = split.next().unwrap();
        /*
        The original sequence is: abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg
        In the original the counts we get are:
        a = 8
        b = 6
        c = 8
        d = 7
        e = 4
        f = 9
        g = 7
        So we uniquely know the positions of e, b, f
        d, g / a, c are narrowed down to 2.
        a, c can be distinguished because only c is in 4, which is trivially discoverable
        Similarly, d, g can be distinguished because only d is in 4.
        */
        let mut counts = vec![0; 7];
        let mut four = None;
        let first: Vec<_> = first
            .split_whitespace()
            .map(|s| {
                let bytes = s.as_bytes();
                // Save the four bit pattern for later
                if bytes.len() == 4 {
                    four = Some(bytes);
                }
                bytes.iter().map(|x| x - b'a')
            })
            .flatten()
            .collect();
        for x in first.iter() {
            counts[*x as usize] += 1;
        }
        // Check that my strategy actually works
        debug_assert!({
            let mut copied = counts.clone();
            copied.sort();
            copied == vec![4, 6, 7, 7, 8, 8, 9]
        });
        let four = four.unwrap();
        for (idx, count) in counts.into_iter().enumerate() {
            let idx = idx as u8;
            match count {
                4 => decoder.set(idx, 4),
                6 => decoder.set(idx, 1),
                7 => {
                    if four.contains(&(idx + b'a')) {
                        // Must be d = 3
                        decoder.set(idx, 3)
                    } else {
                        // Must be g = 6
                        decoder.set(idx, 6)
                    }
                }
                8 => {
                    if four.contains(&(idx + b'a')) {
                        // Must be c = 2
                        decoder.set(idx, 2)
                    } else {
                        // Must be a = 0
                        decoder.set(idx, 0)
                    }
                }
                9 => decoder.set(idx, 5),
                _ => unimplemented!(),
            }
        }
        let last = split.next().unwrap();
        let mut total = 0;
        for x in last.split_whitespace() {
            total *= 10;
            let bytes = x.as_bytes();
            match bytes.len() {
                2 | 4 | 3 | 7 => part1 += 1,
                _ => {}
            }
            let digit = decoder.decode(bytes);
            total += digit;
        }
        part2 += total;
    }
    Ok(Solution::new(part1, part2))
}
