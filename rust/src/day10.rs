use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let numbers = input.lines().map(|l| l.parse().unwrap()).sorted();
    let mut ones = 0;
    let mut threes = 0;
    let mut prev = 0;
    for n in numbers {
        match n - prev {
            1 => ones += 1,
            2 => (),
            3 => threes += 1,
            _ => panic!(),
        }
        prev = n;
    }
    ones * (threes + 1)
}

struct ArrangementsAcc {
    cache: Vec<usize>,
    sequence_length: usize,
    total: usize,
}

impl ArrangementsAcc {
    fn new() -> Self {
        ArrangementsAcc {
            cache: vec![1, 1, 2, 4, 7],
            sequence_length: 1,
            total: 1,
        }
    }

    fn increment(mut self) -> Self {
        self.sequence_length += 1;
        self
    }

    fn split(mut self) -> Self {
        self.total *= self.arrangements(self.sequence_length);
        self.sequence_length = 1;
        self
    }

    fn arrangements(&mut self, n: usize) -> usize {
        if n <= self.cache.len() {
            self.cache[n - 1]
        } else {
            let value =
                self.arrangements(n - 3) + self.arrangements(n - 2) + self.arrangements(n - 1);
            self.cache.push(value);
            value
        }
    }
}
pub fn part2(input: &str) -> usize {
    let numbers = input.lines().map(|l| l.parse().unwrap()).sorted();
    let diffs = itertools::chain(Some(0), numbers)
        .tuple_windows()
        .map(|(a, b)| (b - a));
    itertools::chain(diffs, Some(3))
        .fold(ArrangementsAcc::new(), |acc, diff| match diff {
            1 => acc.increment(),
            3 => acc.split(),
            x => panic!(x),
        })
        .total
}

pub fn part2_alt(input: &str) -> usize {
    let numbers = input.lines().map(|l| l.parse().unwrap()).sorted();
    let diffs = itertools::chain(Some(0), numbers)
        .tuple_windows()
        .map(|(a, b)| (b - a));
    let cache = [1, 1, 2, 4, 7];
    let mut sequence_length = 1;
    let mut total = 1;
    for diff in itertools::chain(diffs, Some(3)) {
        match diff {
            1 => sequence_length += 1,
            3 => {
                total *= cache[sequence_length - 1];
                sequence_length = 1;
            }
            _ => panic!(),
        }
    }
    total
}
