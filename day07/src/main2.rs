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
    println!("{:?}", index);
    let count = index.count_big_bags("shiny gold");
    println!("{}", count);
}

#[derive(Parser)]
#[grammar = "Grammar.pest"]
struct MyRule;

#[derive(Debug)]
struct ReverseIndex {
    map: HashMap<String, HashSet<String>>,
}

impl ReverseIndex {
    fn new() -> ReverseIndex {
        ReverseIndex {
            map: HashMap::new(),
        }
    }
    fn add(&mut self, big: String, smalls: Vec<String>) {
        // let g = self.map.entry(small).or_insert_with(HashSet::new);
        // if !g.contains(&big) {
        //     g.insert(big);
        // }
        smalls.iter().for_each(|small| {
            let g = self
                .map
                .entry(small.to_string())
                .or_insert_with(HashSet::new);
            if !g.contains(&big) {
                g.insert(big.clone());
            }
        });
    }

    fn count_big_bags(&self, small: &str) -> usize {
        let mut bags_to_find: Vec<&str> = vec![small];
        let mut bags_seen: HashSet<&str> = HashSet::new();
        let mut bags_final: HashSet<&str> = HashSet::new();

        while let Some(small) = bags_to_find.pop() {
            // Get one small
            if !self.map.contains_key(small) || bags_seen.contains(&small) {
                continue;
            }

            // Get bigger bags
            let bigger_bags = self.map.get(small).unwrap();

            // Update count
            bags_final.extend(bigger_bags.iter().map(|l| l.as_str()));

            // Update stats
            bags_seen.insert(small);
            bags_to_find.extend(bigger_bags.iter().map(|l| l.as_str()));
        }
        bags_final.len()
    }
    fn from(path: &str) -> ReverseIndex {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut index = ReverseIndex {
            map: HashMap::new(),
        };

        let sian = reader.lines().map(Result::unwrap);

        for line in sian {
            let h = give_me(&line);
            index.add(h.big, h.smalls);
        }

        index
    }
}

#[derive(PartialEq, Debug)]
struct Stuff {
    big: String,
    smalls: Vec<String>,
}

fn give_me(line: &str) -> Stuff {
    let r = MyRule::parse(Rule::Regulation, &line)
        .unwrap()
        .next()
        .unwrap();

    let mut parts = r.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let big = lhs.into_inner().next().unwrap().as_str().to_string();

    let mut smalls = vec![];
    let smalls_iter = rhs.into_inner();

    for small in smalls_iter {
        let mut gr = small.into_inner();
        if gr.next().is_some() {
            smalls.push(gr.next().unwrap().as_str().to_string());
        }
    }
    Stuff { big, smalls }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_big_bags() {
        let index = ReverseIndex::from("./src/test.txt");
        println!("{:?}", index);
        let count = index.count_big_bags("shiny gold");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_give_me() {
        assert_eq!(
            give_me("drab green bags contain 4 dull white bags, 1 posh indigo bag."),
            Stuff {
                big: "drab green".to_string(),
                smalls: vec!["dull white".to_string(), "posh indigo".to_string()],
            }
        );
    }

    #[test]
    fn reverse_index_test() {
        let mut h = ReverseIndex::new();
        h.add(
            "c".to_string(),
            vec!["b".to_string(), "a".to_string(), "d".to_string()],
        );
        h.add(
            "f".to_string(),
            vec!["x".to_string(), "a".to_string(), "c".to_string()],
        );

        let gg = h.map.get("a").unwrap();

        assert!(gg.contains("c"));
        assert!(gg.contains("f"));
    }

    // #[test]
    // fn count() {
    //     let mut h = ReverseIndex::new();
    //     h.add("c", &["b", "a", "d"]);
    //     h.add("f", &["x", "a", "c"]);
    //     // a: {c, f}
    //     // b: {c}
    //     // d: {c}
    //     // c: {f}
    //     // x: {f}

    //     assert_eq!(h.count_big_bags("a"), 3);
    //     assert_eq!(h.count_big_bags("b"), 2);
    //     assert_eq!(h.count_big_bags("c"), 1);
    //     assert_eq!(h.count_big_bags("z"), 0);
    // }

    #[test]
    fn is_regulation() {
        let pairs = MyRule::parse(
            Rule::Regulation,
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        );
        let rule_name = pairs.unwrap().next().unwrap().as_rule();
        assert_eq!(rule_name, Rule::Regulation);
    }

    // #[test]
    // fn parse_reg() {
    //     let mut pairs = MyRule::parse(
    //         Rule::Regulation,
    //         "light red bags contain 1 bright white bag, 2 muted yellow bags.",
    //     )
    //     .unwrap();

    //     let mut level2 = pairs.next().unwrap().into_inner();

    //     let main_bag = level2.next_ba().unwrap();
    //     assert_eq!(main_bag.as_rule(), Rule::MainBagDescription);
    //     let containing_bag = level2.next().unwrap();
    //     assert_eq!(containing_bag.as_rule(), Rule::BagDescriptions);
    // }
}
