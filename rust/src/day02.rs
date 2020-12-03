struct Policy {
    min: usize,
    max: usize,
    c: char,
}

impl Policy {
    fn validate_p1(&self, pass: &str) -> bool {
        let count = pass.chars().filter(|c| *c == self.c).count();
        count >= self.min && count <= self.max
    }

    fn validate_p2(&self, pass: &str) -> bool {
        let pass: Vec<_> = pass.chars().collect();
        (pass[self.min - 1] != pass[self.max - 1])
            && (pass[self.min - 1] == self.c || pass[self.max - 1] == self.c)
    }
}

fn parse_policy_pass(line: &str) -> (Policy, &str) {
    let split: Vec<_> = line.trim().split(":").collect();
    let (policy_str, pass) = (split[0], split[1].trim());
    let pol_split: Vec<_> = policy_str.split(" ").collect();
    let (range, c) = (pol_split[0], pol_split[1].chars().next().unwrap());
    let range: Vec<_> = range.split("-").map(|i| i.parse().unwrap()).collect();
    let policy = Policy {
        min: range[0],
        max: range[1],
        c,
    };
    (policy, pass)
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_policy_pass)
        .filter(|(pol, pass)| pol.validate_p1(pass))
        .count()
}
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_policy_pass)
        .filter(|(pol, pass)| pol.validate_p2(pass))
        .count()
}
