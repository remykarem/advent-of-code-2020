use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// https://riptutorial.com/rust/example/4149/create-a-hashset-macro
macro_rules! set {
    ($($x:expr),*) => {
        {
            let mut tmp = HashSet::new();
            $(tmp.insert($x);)*
            tmp
        }
    };
}

fn main() {
    let file = File::open("./src/input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let nums: Vec<i32> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<i32>())
        .map(Result::unwrap)
        .collect();

    println!(
        "Two sum: {}",
        two_sum(2020, &nums).unwrap().iter().product::<i32>()
    );
    println!(
        "Three sum: {}",
        three_sum(2020, &nums).unwrap().iter().product::<i32>()
    );
}

fn two_sum(sum: i32, nums: &[i32]) -> Option<HashSet<i32>> {
    if nums.len() < 2 {
        panic!("At least 2 items needed")
    }

    let mut candidates = HashSet::new();

    for &num in nums {
        let target = sum - num;
        if candidates.contains(&target) {
            return Some(set![num, target]);
        } else {
            candidates.insert(num);
        }
    }
    None
}

fn three_sum(sum: i32, nums: &[i32]) -> Option<i32> {
    let len = nums.len();
    if len < 3 {
        panic!("At least 3 items needed")
    }

    let mut prod = 1;

    for (i, &context) in nums.iter().enumerate() {
        let mut candidates = HashSet::new();

        for &num in nums[(i + 1)..len].iter() {
            let target = sum - context - num;
            if candidates.contains(&target) {
                return Some(set![num, context, target]);
            } else {
                candidates.insert(num);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(three_sum(14, &[1, 9, 4, 3]), Some(set![1, 9, 4]));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(13, &[1, 9, 4, 3]), Some(set![9, 4]));
    }
}
