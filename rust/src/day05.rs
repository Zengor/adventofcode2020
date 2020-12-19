#[allow(dead_code)]
fn code_to_int(upper: char, lower: char, code: &str) -> u32 {
    let s: String = code
        .chars()
        .map(|c| match c {
            c if c == upper => '1',
            c if c == lower => '0',
            _ => panic!(),
        })
        .collect();
    u32::from_str_radix(&s, 2).unwrap()
}

fn code_to_bits(upper: char, lower: char, code: &str) -> u32 {
    let len = code.len();
    code.char_indices().fold(0, |acc, (i, c)| match c {
        c if c == upper => (1 << (len - i - 1)) | acc,
        c if c == lower => acc,
        _ => panic!(),
    })
}

fn make_id(line: &str) -> u32 {
    let (row_s, column_s) = line.trim().split_at(7);
    let row = code_to_bits('B', 'F', row_s);
    let col = code_to_bits('R', 'L', column_s);
    (row * 8) + col
}

pub fn part1(input: &str) -> u32 {
    let mut highest = 0;
    for line in input.lines() {
        let id = make_id(line);
        if id > highest {
            highest = id;
        }
    }
    highest
}

pub fn part2(input: &str) -> u32 {
    use itertools::Itertools;
    let list: Vec<_> = input.lines().map(make_id).sorted().collect();
    let (low, high) = (list[0], list.last().unwrap());
    for n in low + 1..*high {
        if !list.contains(&n) {
            return n;
        }
    }
    println!("not found");
    0
}
