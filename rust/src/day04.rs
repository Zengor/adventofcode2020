use std::collections::HashSet;
//const FIELDS: &[str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

pub fn part1(input: &str) -> u32 {
    let passports = input.split("\n\n").map(|p| p.split_whitespace());
    let mut valid = 0;
    for passport in passports {
        let mut count = 0;
        for entry in passport {
            let (field, _) = entry.split_at(3);
            if field != "cid" {
                count += 1;
            }
        }
        if count == 7 {
            valid += 1;
        }
    }
    valid
}
pub fn is_valid(field: &str, value: &str) -> bool {
    todo!()
}

pub fn part2(input: &str) -> u32 {
    let passports = input.split("\n\n").map(|p| p.split_whitespace());
    let mut valid = 0;
    let ecl_matches: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    let is_between = |n: &str, a, b| n.parse::<u32>().map_or(false, |i| (a..=b).contains(&i));
    for passport in passports {
        let valid_entries = passport.filter(|entry| {
            let (field, value) = entry.split_at(3);
            let value = value.trim_start_matches(":");
            match field {
                "byr" => is_between(value, 1920, 2002),
                "iyr" => is_between(value, 2010, 2020),
                "eyr" => is_between(value, 2020, 2030),
                "hgt" => {
                    let (num, sfx) = value.split_at(value.len() - 2);
                    (sfx == "cm" && is_between(num, 150, 193))
                        || (sfx == "in" && is_between(num, 59, 76))
                }
                "hcl" => {
                    value.len() == 7
                        && value
                            .strip_prefix("#")
                            .map(|v| v.chars().all(char::is_alphanumeric))
                            .unwrap_or(false)
                }
                "ecl" => ecl_matches.contains(value),
                "pid" => value.len() == 9 && value.parse::<u32>().is_ok(),
                "cid" => false, // intentionally ignored, so not even added to count
                _ => false,
            }
        });
        if valid_entries.count() == 7 {
            valid += 1;
        }
    }
    valid
}
