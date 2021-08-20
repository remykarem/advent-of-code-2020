use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Split;

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    rule: String,
    pattern: String,
}
impl PasswordPolicy {
    fn validate_qn1(&self, password: &str) -> bool {
        let mut rule_splits = self.rule.split('-');
        let (min, max) = get_next_two(&mut rule_splits);
        let count = password.matches(&self.pattern).count();

        min <= count && count <= max
    }
    fn validate_qn2(&self, password: &str) -> bool {
        let mut rule_splits = self.rule.split('-');
        let (mut pos1, mut pos2) = get_next_two(&mut rule_splits);

        // adjustment
        pos1 -= 1;
        pos2 -= 1;

        (password[pos1..pos1 + 1] == self.pattern) ^ (password[pos2..pos2 + 1] == self.pattern)
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("cannot open");
    let reader = BufReader::new(file);

    let pairs: Vec<(PasswordPolicy, String)> = reader
        .lines()
        .map(Result::unwrap)
        .map(parse_policy_and_password)
        .collect();

    let num_valid_part_1 = pairs
        .iter()
        .filter(|(password_policy, password)| password_policy.validate_qn1(password))
        .count();
    let num_valid_part_2 = pairs
        .iter()
        .filter(|(password_policy, password)| password_policy.validate_qn2(password))
        .count();

    println!("Part 1: {}", num_valid_part_1);
    println!("Part 2: {}", num_valid_part_2);
}

fn parse_policy_and_password(line: String) -> (PasswordPolicy, String) {
    let mut parts = line.split_whitespace();
    let rule = parts.next().unwrap().to_string();
    let pattern = parts.next().unwrap()[0..1].to_string();
    let password = parts.next().unwrap().to_string();

    (PasswordPolicy { rule, pattern }, password)
}

fn get_next_two(rule_splits: &mut Split<char>) -> (usize, usize) {
    let min = rule_splits.next().unwrap().parse::<usize>().unwrap();
    let max = rule_splits.next().unwrap().parse::<usize>().unwrap();
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (policy, password) = parse_policy_and_password("2-8 h: hhqqvhhphhhqddhh".into());
        assert_eq!(
            policy,
            PasswordPolicy {
                rule: "2-8".into(),
                pattern: "h".into()
            }
        );
        assert_eq!(password, "hhqqvhhphhhqddhh");
    }

    #[test]
    fn it_works() {
        assert!(PasswordPolicy {
            rule: "1-3".into(),
            pattern: "a".into()
        }
        .validate_qn1("abcde"));

        assert!(!PasswordPolicy {
            rule: "1-3".into(),
            pattern: "b".into()
        }
        .validate_qn1("cdefg"));

        assert!(PasswordPolicy {
            rule: "2-9".into(),
            pattern: "c".into()
        }
        .validate_qn1("ccccccccc"));
    }
}
