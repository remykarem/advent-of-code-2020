use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DELIMITER: &str = ":";
lazy_static! {
    static ref ALL_FIELDS: HashSet<&'static str> =
        ["byr", "eyr", "iyr", "hgt", "pid", "hcl", "ecl"]
            .iter()
            .cloned()
            .collect();
}

pub fn main() {
    let mut num_valid = 0;

    let file = File::open("./src/input.txt").expect("cannot open file");
    let reader = BufReader::new(file);

    let mut passport = String::from("");
    reader.lines().map(Result::unwrap).for_each(|line| {
        if line.is_empty() {
            // Process the buffer
            if check_passport2(&passport) {
                num_valid += 1;
            }
            passport.clear();
        } else {
            passport.push(' ');
            passport.push_str(&line);
        }
    });

    if check_passport2(&passport) {
        num_valid += 1;
        passport.clear();
    }

    println!("{}", num_valid);
}

fn check_passport(passport: &str) -> bool {
    let fields: HashSet<&str> = passport
        .split_whitespace()
        .map(|field| (*field).split(DELIMITER).next().expect("problem"))
        .filter(|field| *field != "cid")
        .collect();
    ALL_FIELDS.is_subset(&fields)
}

fn check_passport2(passport: &str) -> bool {
    let mut available_fields: HashSet<&str> = HashSet::new();

    let fields = passport.split_whitespace();
    for field in fields {
        let mut field_iter = (*field).split(DELIMITER);
        let field_label = field_iter.next().expect("problem");
        let field_value = field_iter.next().expect("problem");

        let valid = match field_label {
            "byr" => validate_byr(field_value),
            "eyr" => validate_eyr(field_value),
            "iyr" => validate_iyr(field_value),
            "hgt" => validate_hgt(field_value),
            "pid" => validate_pid(field_value),
            "hcl" => validate_hcl(field_value),
            "ecl" => validate_ecl(field_value),
            "cid" => true,
            _ => panic!("unknown field: {}", field_label),
        };
        if !valid {
            return false;
        };

        available_fields.insert(field_label);
    }
    if !ALL_FIELDS.is_subset(&available_fields) {
        return false;
    }
    true
}

fn validate_byr(input: &str) -> bool {
    let year = input.parse::<i32>().unwrap();
    (1920..=2002).contains(&year)
}
fn validate_iyr(input: &str) -> bool {
    let year = input.parse::<i32>().unwrap();
    year >= 2010 && year <= 2020
}
fn validate_eyr(input: &str) -> bool {
    let year = input.parse::<i32>().unwrap();
    year >= 2020 && year <= 2030
}
fn validate_hgt(input: &str) -> bool {
    if input.ends_with("cm") {
        let height = input[0..input.len() - 2].parse::<i32>().unwrap();
        height >= 150 && height <= 193
    } else if input.ends_with("in") {
        let height = input[0..input.len() - 2].parse::<i32>().unwrap();
        height >= 59 && height <= 76
    } else {
        false
    }
}
fn validate_hcl(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(input)
}
fn validate_pid(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(input)
}
fn validate_ecl(input: &str) -> bool {
    lazy_static! {
        static ref ARR: &'static [&'static str] =
            &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    }
    ARR.contains(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validating_byr() {
        assert!(validate_byr("1920"));
        assert!(validate_byr("2002"));
        assert!(validate_byr("2000"));
        assert!(!validate_byr("2020"));
    }

    #[test]
    fn validating_iyr() {
        assert!(validate_iyr("2010"));
        assert!(validate_iyr("2020"));
        assert!(validate_iyr("2015"));
        assert!(!validate_iyr("2030"));
    }

    #[test]
    fn validating_eyr() {
        assert!(validate_eyr("2020"));
        assert!(validate_eyr("2030"));
        assert!(validate_eyr("2025"));
        assert!(!validate_eyr("2050"));
    }

    #[test]
    fn validating_hgt() {
        assert!(validate_hgt("150cm"));
        assert!(validate_hgt("193cm"));
        assert!(validate_hgt("180cm"));
        assert!(validate_hgt("76in"));
        assert!(validate_hgt("59in"));
        assert!(!validate_hgt("gibberish"));
    }

    #[test]
    fn validating_pid() {
        assert!(validate_pid("123456789"));
        assert!(!validate_pid("123"));
    }

    #[test]
    fn validating_hcl() {
        assert!(validate_hcl("#003366"));
        assert!(validate_hcl("#00f14e"));
        assert!(!validate_hcl("#1234567"));
        assert!(!validate_hcl("1234567"));
        assert!(!validate_hcl("#gggggg"));
    }

    #[test]
    fn validating_ecl() {
        assert!(validate_ecl("amb"));
        assert!(!validate_ecl("AMB"));
    }

    #[test]
    fn check_passports2() {
        assert!(check_passport2(
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        ));
        assert!(check_passport2(
            r#"eyr:2029 ecl:blu cid:129 byr:1989
    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"#
        ));
        assert!(check_passport2(
            r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
    hcl:#623a2f"#
        ));

        assert!(!check_passport2(
            r#"eyr:1972 cid:100
    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"#
        ));
        assert!(!check_passport2(
            r#"iyr:2019
    hcl:#602927 eyr:1967 hgt:170cm
    ecl:grn pid:012533040 byr:1946"#
        ));
        assert!(!check_passport2(
            r#"hcl:dab227 iyr:2012
    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"#
        ));
        assert!(!check_passport2(
            r#"hgt:59cm ecl:zzz
    eyr:2038 hcl:74454a iyr:2023
    pid:3556412378 byr:2007"#
        ));

        assert!(check_passport(
            r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm"#
        ));
        assert!(!check_passport(
            r#"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929"#
        ));
        assert!(check_passport(
            r#"hcl:#ae17e1 iyr:2013
                eyr:2024
                ecl:brn pid:760753108 byr:1931
                hgt:179cm"#
        ));
        assert!(!check_passport(
            r#"hcl:#cfa07d eyr:2025 pid:166559648
                iyr:2011 ecl:brn hgt:59in"#,
        ));
    }
}
