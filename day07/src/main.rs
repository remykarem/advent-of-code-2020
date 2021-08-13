extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_imports)]
use pest::Parser;

fn main() {
    let index = ReverseIndex::from("./src/input.txt");
    let count = index.count_big_bags("small");
    println!("{}", count);
}

#[derive(Parser)]
#[grammar = "Grammar.pest"]
struct MyRule;

#[derive(Debug)]
struct ReverseIndex<'a> {
    map: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> ReverseIndex<'a> {
    fn new() -> ReverseIndex<'a> {
        ReverseIndex {
            map: HashMap::new(),
        }
    }
    fn add(&mut self, big: &'a str, smalls: &[&'a str]) {
        smalls.iter().for_each(|&small| {
            let g = self.map.entry(small).or_insert_with(HashSet::new);
            if !g.contains(big) {
                g.insert(big);
            }
        });
    }
    fn from(path: &str) -> ReverseIndex<'a> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        reader.lines().map(Result::unwrap).map(|line| {
            let r = MyRule::parse(Rule::Regulation, &line).unwrap();
            
        });

        let mut pairs = MyRule::parse(
            Rule::Regulation,
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        )
        .unwrap();
        ReverseIndex {
            map: HashMap::new(),
        }
    }
    fn count_big_bags(&self, small: &str) -> usize {
        let mut count = 0;
        let mut to_find: Vec<&str> = vec![small];
        let mut seen: HashSet<&str> = HashSet::new();

        while let Some(small) = to_find.pop() {
            // Get one small
            if !self.map.contains_key(small) {
                continue;
            }

            // Get big bags
            let bigs = self.map.get(small).unwrap();

            // Update count
            count += bigs.len();

            // Update stats
            seen.insert(small);
            to_find.extend(bigs);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_index_test() {
        let mut h = ReverseIndex::new();
        h.add("c", &["b", "a", "d"]);
        h.add("f", &["x", "a", "c"]);

        let gg = h.map.get("a").unwrap();

        assert!(gg.contains("c"));
        assert!(gg.contains("f"));
    }

    #[test]
    fn count() {
        let mut h = ReverseIndex::new();
        h.add("c", &["b", "a", "d"]);
        h.add("f", &["x", "a", "c"]);
        // a: {c, f}
        // b: {c}
        // d: {c}
        // c: {f}
        // x: {f}

        assert_eq!(h.count_big_bags("a"), 3);
        assert_eq!(h.count_big_bags("b"), 2);
        assert_eq!(h.count_big_bags("c"), 1);
        assert_eq!(h.count_big_bags("z"), 0);
    }

    #[test]
    fn is_regulation() {
        let pairs = MyRule::parse(
            Rule::Regulation,
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        );
        let rule_name = pairs.unwrap().next().unwrap().as_rule();
        assert_eq!(rule_name, Rule::Regulation);
    }

    #[test]
    fn parse_reg() {
        let mut pairs = MyRule::parse(
            Rule::Regulation,
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        )
        .unwrap();

        let mut level2 = pairs.next().unwrap().into_inner();

        let main_bag = level2.next().unwrap();
        assert_eq!(main_bag.as_rule(), Rule::MainBagDescription);
        let containing_bag = level2.next().unwrap();
        assert_eq!(containing_bag.as_rule(), Rule::BagDescriptions);
    }
}
