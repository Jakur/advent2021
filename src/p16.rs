use super::*;

const BITS: usize = 8;

enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        id: u8,
        children: Vec<Packet>,
    },
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operator {
                version: _,
                id,
                children,
            } => {
                let mut iter = children.iter().map(|x| x.evaluate());
                match id {
                    0 => iter.sum(),
                    1 => iter.product(),
                    2 => iter.min().unwrap(),
                    3 => iter.max().unwrap(),
                    5 => {
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        (a > b) as u64
                    }
                    6 => {
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        (a < b) as u64
                    }
                    7 => {
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        (a == b) as u64
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
    fn parse(data: &mut BitStorage) -> Self {
        const LITERAL: u8 = 4;
        let version = data.fetch(3) as u8;
        let id = data.fetch(3) as u8;
        if id == LITERAL {
            let mut value = 0;
            loop {
                value = value << 4;
                let next = data.fetch(5) as u64;
                value |= next & 0b1111;
                if (next & 0b1_0000) == 0 {
                    break;
                }
            }
            Packet::Literal { version, value }
        } else {
            let mut children = Vec::new();
            let length_type = data.fetch(1) as u8;
            if length_type == 0 {
                // Total bits of children
                let total_bits = data.fetch(15) as usize;
                let starting = data.data_ptr;
                while data.data_ptr - starting < total_bits {
                    children.push(Packet::parse(data));
                }
            } else {
                // Number of direct children
                let direct_children = data.fetch(11);
                for _ in 0..direct_children {
                    children.push(Packet::parse(data));
                }
            }
            Packet::Operator {
                version,
                id,
                children,
            }
        }
    }
    fn version_total(&self) -> u64 {
        match self {
            Packet::Literal { version, value: _ } => *version as u64,
            Packet::Operator {
                version,
                children,
                id: _,
            } => *version as u64 + children.iter().map(|x| x.version_total()).sum::<u64>(),
        }
    }
}

struct BitStorage {
    data_ptr: usize,
    data: Vec<u8>,
}

impl BitStorage {
    fn new(data: Vec<u8>) -> Self {
        Self { data_ptr: 0, data }
    }

    pub fn fetch(&mut self, length: usize) -> u32 {
        let st_idx = self.data_ptr / BITS;
        let end = self.data_ptr + length;
        self.data_ptr += length;
        let end_idx = end / BITS;
        let end_offset = end % BITS;
        let idx_diff = end_idx - st_idx;
        match idx_diff {
            0 => {
                let mask = (1 << length) - 1;
                let value = (self.data[end_idx] as u32) >> (BITS - end_offset);
                value & mask
            }
            1 => {
                let length1 = length - end_offset;
                let mask1 = (1 << length1) - 1;
                let high = (self.data[st_idx] as u32) & mask1;
                // Should be no need to mask low
                let low = (self.data[end_idx] as u32) >> (BITS - end_offset);
                // We want the last BITS - st_offset bits of the first
                // We want the first end_offset of the second
                (high << end_offset) | low
            }
            2 => {
                let length1 = length - end_offset - 8;
                let mask1 = (1 << length1) - 1;
                let high = (self.data[st_idx] as u32) & mask1;
                let middle = self.data[st_idx + 1] as u32;
                // Should be no need to mask low
                let low = (self.data[end_idx] as u32) >> (BITS - end_offset);
                // Same as the previous, except we have an extra 8 bits in the middle
                (high << end_offset + 8) | (middle << end_offset) | low
            }
            _ => unimplemented!(),
        }
    }
}

fn parse_hex(val: u8) -> u8 {
    if val >= b'A' {
        val + 10 - b'A'
    } else {
        val - b'0'
    }
}

fn transform_input(arr: &[u8]) -> Vec<u8> {
    arr.chunks(2)
        .map(|x| {
            let high = parse_hex(x[0]);
            let low = parse_hex(x[1]);
            (high << 4) | low
        })
        .collect()
}

pub fn solve(input: &[u8]) -> Solution<u64, u64> {
    let vec = transform_input(&input[0..input.len() - 1]); // Avoid newline
    let mut bits = BitStorage::new(vec);
    let root = Packet::parse(&mut bits);
    Solution::new(root.version_total(), root.evaluate())
}
