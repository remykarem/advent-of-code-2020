use std::fs::File;
use std::io::{BufRead, BufReader};

struct SubgraphPathsCounter<'a> {
    buffer: Vec<u64>,
    num_paths: Vec<u64>,
    nums: &'a [u64],
}
impl<'a> SubgraphPathsCounter<'a> {
    fn new(nums: &[u64]) -> SubgraphPathsCounter {
        SubgraphPathsCounter {
            buffer: vec![],
            num_paths: vec![],
            nums,
        }
    }
    fn step(&mut self, x: u64) {
        if x == 3 {
            self.process_buffer();
            self.buffer.clear();
        } else {
            self.buffer.push(x);
        }
    }
    fn get_num_permutations(&mut self) -> u64 {
        for num in self.nums {
            self.step(*num);
        }
        self.num_paths.iter().fold(1, |acc, x| acc * *x)
    }
    fn process_buffer(&mut self) {
        if self.buffer == vec![1, 1, 1] {
            self.num_paths.push(4);
        } else if self.buffer == vec![1, 2]
            || self.buffer == vec![2, 1]
            || self.buffer == vec![1, 1]
        {
            self.num_paths.push(2);
        } else if self.buffer == vec![1] || self.buffer.is_empty() {
            self.num_paths.push(1);
        } else if self.buffer == vec![1,1,1,1,] {
            self.num_paths.push(7);
        } else {
            panic!("invalid buffer {:?}", self.buffer);
        }
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("failed to open file");
    let reader = BufReader::new(file);

    let mut nums: Vec<u64> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| str::parse::<u64>(line.as_str()))
        .map(Result::unwrap)
        .collect();
    nums.sort_unstable();

    let mut yo: Vec<u64> = vec![];
    let mut prev = &0;
    for num in nums.iter() {
        yo.push(num - prev);
        prev = num;
    }
    yo.push(3);

    let mut counter = SubgraphPathsCounter::new(&yo);
    let count = counter.get_num_permutations();
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let nums = vec![1, 3, 1, 1, 1, 3, 1, 1, 3, 1, 3, 3];
        let mut counter = SubgraphPathsCounter::new(&nums);
        let count = counter.get_num_permutations();
        assert_eq!(count, 8);
    }
}