use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Count = usize;

struct GlobalCounter {
    buffer: HashSet<char>,
    count: Count,
}

impl GlobalCounter {
    fn add(&mut self, qns: String) {
        qns.chars().for_each(|qn| {
            self.buffer.insert(qn);
        });
    }
    fn reset(&mut self) {
        self.count += self.buffer.len();
        self.buffer.clear();
    }
    fn new() -> GlobalCounter {
        GlobalCounter {
            buffer: HashSet::new(),
            count: 0,
        }
    }
}

struct FamilyCounter {
    data: HashMap<char, Count>,
    family_size: Count,
}

impl FamilyCounter {
    fn add(&mut self, qns: String) {
        qns.chars().for_each(|qn| {
            let counter = self.data.entry(qn).or_insert(0);
            *counter += 1;
        });
        self.family_size += 1;
    }
    fn reset(&mut self) {
        self.data.clear();
        self.family_size = 0;
    }
    fn get_count(&self) -> Count {
        self.data
            .values()
            .filter(|&&count| count == self.family_size)
            .count()
    }
    fn new() -> FamilyCounter {
        FamilyCounter {
            data: HashMap::new(),
            family_size: 0,
        }
    }
}

fn main() {
    let path = "./src/input.txt";
    part1(path);
    part2(path);
}

fn part1(path: &str) {
    let file = File::open(path).expect("cannot find file");
    let reader = BufReader::new(file);

    let mut counter = GlobalCounter::new();
    reader
        .lines()
        .map(Result::unwrap)
        .for_each(|line| match line.as_str() {
            "" => counter.reset(),
            _ => counter.add(line),
        });
    counter.reset();

    println!("Part 1: {}", counter.count);
}

fn part2(path: &str) {
    let file = File::open(path).expect("cannot find file");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut counter = FamilyCounter::new();
    reader
        .lines()
        .map(Result::unwrap)
        .for_each(|line| match line.as_str() {
            "" => {
                count += counter.get_count();
                counter.reset()
            }
            _ => counter.add(line),
        });
    count += counter.get_count();

    println!("Part 2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_counter() {
        let mut g = GlobalCounter::new();
        g.add("sdffv".to_owned());

        assert!(g.buffer.contains(&'s'));
        assert!(g.buffer.contains(&'d'));
        assert!(g.buffer.contains(&'f'));
        assert!(g.buffer.contains(&'v'));
        assert!(g.count == 0);

        g.reset();

        assert!(g.count == 4);
    }

    #[test]
    fn check_family_counter() {
        let mut g = FamilyCounter::new();
        g.add("sdffv".to_owned());
        assert!(g.data[&'s'] == 1);
        assert!(g.data[&'d'] == 1);
        assert!(g.data[&'f'] == 2);
        assert!(g.data[&'v'] == 1);
    }
}
