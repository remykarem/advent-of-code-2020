use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FlexibleWindow<'a> {
    sum: u64,
    left: usize,
    right: usize,
    arr: &'a [u64],
}

impl<'a> FlexibleWindow<'a> {
    fn search(&mut self, tgt: u64) -> (usize, usize) {
        while self.sum != tgt {
            if self.sum > tgt {
                self.left_advance();
            } else {
                self.right_advance();
            }
        }
        (self.left, self.right)
    }
    fn left_advance(&mut self) {
        if self.left > self.arr.len() {
            panic!("left is gte arr.len()");
        }
        self.sum -= *self.arr.get(self.left).unwrap();
        self.left += 1;
    }
    fn right_advance(&mut self) {
        if self.right >= self.arr.len() {
            panic!("right is gte arr.len()");
        }

        self.sum += *self.arr.get(self.right + 1).unwrap();
        self.right += 1;
    }
    fn from(arr: &[u64]) -> FlexibleWindow {
        if arr.is_empty() {
            panic!("empty array");
        }

        let sum = *arr.get(0).unwrap();
        FlexibleWindow {
            sum,
            left: 0,
            right: 0,
            arr,
        }
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("cannot open file");
    let reader = BufReader::new(file);

    let nums: Vec<u64> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| str::parse::<u64>(line.as_str()).unwrap())
        .collect();

    let mut window = FlexibleWindow::from(&nums);
    let (min_idx, max_idx) = window.search(20874512);
    println!("lower id: {}, upper id: {}", min_idx, max_idx);
    let min = nums[min_idx..=max_idx].iter().min().unwrap();
    let max = nums[min_idx..=max_idx].iter().max().unwrap();
    println!("min: {}, max: {}", min, max);
    println!("sum: {}", min+max);

    // let reader = BufReader::new(file);

    // let mut buffer: VecDeque<u64> = VecDeque::new();

    // let mut lines = reader.lines();

    // for _ in 1..=25 {
    //     let l = lines.next().unwrap().unwrap();
    //     buffer.push_back(str::parse::<u64>(l.as_str()).unwrap());
    // }

    // println!("{:?}", buffer);

    // lines.map(Result::unwrap).for_each(|line| {
    //     let num = str::parse::<u64>(line.as_str()).unwrap();
    //     if validate_two_sum(num, &buffer) {
    //         buffer.pop_front();
    //         buffer.push_back(num);
    //     } else {
    //         panic!("{}", num);
    //     }
    // });
}

fn validate_two_sum(sum: u64, nums: &VecDeque<u64>) -> bool {
    let mut candidates: HashSet<&u64> = HashSet::new();
    for num in nums {
        if candidates.contains(&(sum - num)) {
            return true;
        } else {
            candidates.insert(&num);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_min_max2() {
        let nums: Vec<u64> = [7, 13, 2, 5, 9, 1].to_vec();
        let mut window = FlexibleWindow::from(&nums);

        let (min_idx, max_idx) = window.search(30);
        assert_eq!((min_idx, max_idx), (1, 5));
        let min = nums[min_idx..=max_idx].iter().min().unwrap();
        let max = nums[min_idx..=max_idx].iter().max().unwrap();
        assert_eq!((*min, *max), (1, 13));
    }

    #[test]
    fn test_search_min_max() {
        let nums: Vec<u64> = [20, 15, 25, 50, 40].to_vec();
        let mut window = FlexibleWindow::from(&nums);

        let (min_idx, max_idx) = window.search(130);
        assert_eq!((min_idx, max_idx), (1, 4));
        let min = nums[min_idx..=max_idx].iter().min().unwrap();
        let max = nums[min_idx..=max_idx].iter().max().unwrap();
        assert_eq!((*min, *max), (15, 50));
    }

    #[test]
    fn test_search() {
        let nums: Vec<u64> = [20, 15, 25, 47, 40].to_vec();
        let mut window = FlexibleWindow::from(&nums);

        assert_eq!(window.search(20), (0, 0));
        assert_eq!(window.search(35), (0, 1));
        assert_eq!(window.search(60), (0, 2));
        assert_eq!(window.search(107), (0, 3));
        assert_eq!(window.search(87), (1, 3));
    }

    #[test]
    fn test_window() {
        let nums: Vec<u64> = [20, 15, 25, 47, 40].to_vec();
        let mut window = FlexibleWindow::from(&nums);
        assert_eq!(window.sum, 20);

        window.right_advance();
        assert_eq!(window.sum, 35);

        window.right_advance();
        assert_eq!(window.sum, 60);

        window.left_advance();
        assert_eq!(window.sum, 40);
    }

    #[test]
    fn test_validate_two_sum() {
        let nums: VecDeque<u64> = [20, 15, 25, 47, 40].iter().cloned().collect();
        assert!(validate_two_sum(62, &nums));
        assert!(!validate_two_sum(63, &nums));
    }
    #[test]
    fn test_validate_two_sum2() {
        let nums: VecDeque<u64> = [127, 219, 299, 277, 309].iter().cloned().collect();
        assert!(validate_two_sum(576, &nums));
        assert!(!validate_two_sum(577, &nums));
    }

    #[test]
    fn test_ever_growing_nums() {
        let mut nums: VecDeque<u64> = [182, 127, 219, 299, 277].iter().cloned().collect();
        assert!(validate_two_sum(309, &nums));

        nums.pop_front();
        nums.push_back(309);

        let nums: VecDeque<u64> = [127, 219, 299, 277, 309].iter().cloned().collect();
        assert!(validate_two_sum(576, &nums));
    }
}
