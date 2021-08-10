use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

pub fn main() {
    assert!(is_valid_qn1("1-3", "a", "abcde"));
    assert!(!is_valid_qn1("1-3", "b", "cdefg"));
    assert!(is_valid_qn1("2-9", "c", "ccccccccc"));

    let file = File::open("./data/day02.txt").expect("cannot open");
    let reader = BufReader::new(file);

    let num_valid = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| split_line(&line))
        .filter(|(rule, pattern, password)| is_valid_qn1(rule, pattern, password))
        .count();

    println!("{}", num_valid);
}

fn split_line(line: &str) -> (String, String, String) {
    let mut parts = line.split(' ');
    (parts.next().unwrap().to_owned(), parts.next().unwrap().to_owned(), parts.next().unwrap().to_owned())
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
    (min, max)
}
