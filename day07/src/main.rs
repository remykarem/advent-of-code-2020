use std::collections::{HashMap, HashSet};

fn main() {
    let index = ReverseIndex::from("input.txt");
    let count = index.count_big_bags("small");

    println!("{}", count);
}

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
        ReverseIndex {
            map: HashMap::new(),
        }
    }
    fn count_big_bags(&self, small: &str) -> usize {
        1
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
}
