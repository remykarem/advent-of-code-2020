use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

pub fn main() {
    assert_eq!(is_valid_qn1("1-3", "a", "abcde"), true);
    assert_eq!(is_valid_qn1("1-3", "b", "cdefg"), false);
    assert_eq!(is_valid_qn1("2-9", "c", "ccccccccc"), false);

    let file = File::open("./data/day02.txt").expect("cannot open");
    let reader = BufReader::new(file);

    let mut num_valid = 0;
    reader.lines().for_each(|l| {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();

        let rule = parts[0];
        let pattern = parts[1];
        let password = parts[2];

        if is_valid_qn2(rule, &pattern[0..1], password) {
            num_valid += 1;
        }
    });

    println!("{}", num_valid);
}

fn is_valid_qn1(rule: &str, pattern: &str, password: &str) -> bool {
    let mut rule_splits = rule.split('-');
    let (min, max) = get_next_two(&mut rule_splits);
    let count = password.matches(pattern).count();
    
    min <= count && count <= max
}

fn is_valid_qn2(rule: &str, pattern: &str, password: &str) -> bool {
    let mut rule_splits = rule.split('-');
    let (mut pos1, mut pos2) = get_next_two(&mut rule_splits);

    // adjustment
    pos1 -= 1;
    pos2 -= 1;

    (&password[pos1..pos1 + 1] == pattern) ^ (&password[pos2..pos2 + 1] == pattern)
}

fn get_next_two(rule_splits: &mut Split<char>) -> (usize, usize) {
    let min = rule_splits.next().unwrap().parse::<usize>().unwrap();
    let max = rule_splits.next().unwrap().parse::<usize>().unwrap();
    return (min, max);
}