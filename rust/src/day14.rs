use std::collections::HashMap;

#[derive(Default)]
struct Mask {
    ones: usize,
    zeros: usize,
    xs: Vec<usize>,
}
impl Mask {
    fn parse_mask(&mut self, raw: &str) {
        self.ones = 0;
        self.zeros = 0;
        self.xs.clear();
        for (i, c) in raw.chars().rev().enumerate() {
            match c {
                '1' => self.ones |= 1 << i,
                '0' => self.zeros |= 1 << i,
                'X' => self.xs.push(1 << i),
                _ => panic!(),
            }
        }
    }

    fn apply_to_value(&self, value: usize) -> usize {
        (value | self.ones) & !self.zeros
    }

    fn apply_to_address(&self, address: usize) -> impl Iterator<Item = usize> + '_ {
        let address = address | self.ones;
        (0..2usize.pow(self.xs.len() as u32)).map(move |variation| {
            self.xs
                .iter()
                .enumerate()
                .fold(address, |address, (i, bit)| {
                    // if the bit is part of this variation
                    if variation & (1 << i) != 0 {
                        // include it
                        address | bit
                    } else {
                        // if not, exclude it
                        address & !bit
                    }
                })
        })
    }
}

fn parse_mem(raw: &str) -> (usize, usize) {
    let pos: usize = raw[4..raw.find(']').unwrap()].parse().unwrap();
    let value = raw[raw.find('=').unwrap() + 1..].trim().parse().unwrap();
    (pos, value)
}

pub fn part1(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = Mask::default();

    for line in input.lines() {
        if line.starts_with("mask") {
            mask.parse_mask(line[7..].trim());
        } else {
            let (pos, value) = parse_mem(line);
            *memory.entry(pos).or_default() = mask.apply_to_value(value);
        }
    }
    memory.values().sum()
}
pub fn part2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = Mask::default();
    for line in input.lines() {
        if line.starts_with("mask") {
            mask.parse_mask(line[7..].trim());
        } else {
            let (pos, value) = parse_mem(line);

            for address in mask.apply_to_address(pos) {
                *memory.entry(address).or_default() = value;
            }
        }
    }
    memory.values().sum()
}
