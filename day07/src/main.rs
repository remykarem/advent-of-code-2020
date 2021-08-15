extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(unused_imports)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "Grammar.pest"]
struct MyRule;

type Colour = String;
type Qty = usize;
type Child = (Qty, Colour);


fn main() {
    let mut to_find: HashMap<Colour, Vec<Child>> = HashMap::new();
    let mut table: HashMap<Colour, Qty> = HashMap::new();

    let file = File::open("./src/input.txt").expect("problem reading file");
    let reader = BufReader::new(file);

    reader.lines().map(Result::unwrap).for_each(|line| {
        let (h, a) = give_me(&line);
        if a.is_empty() {
            table.insert(h, 0);
        } else {
            to_find.insert(h, a);
        }
    });

    println!("{:?}", table);
    // println!("{:?}", to_find);

    let c = f("shiny gold", &mut table, &mut to_find);
    println!("{:?}", c);
}

fn f(
    colour: &str,
    table: &mut HashMap<Colour, Qty>,
    to_find: &mut HashMap<Colour, Vec<Child>>,
) -> usize {
    if table.contains_key(colour) {
        return *table.get(colour).unwrap();
    }

    let mut count = 0;

    // CLoNe?!
    let children = to_find.get(colour).unwrap().clone();

    for child in children {
        let (qty, child_colour) = child;

        let child_count = f(&child_colour, table, to_find);

        if child_count == 0 {
            count += qty;
        } else {
            count += qty + qty * child_count;
        }
    }

    // Update memoisation
    table.insert(colour.into(), count);

    // Update to_find
    to_find.remove(colour);

    count
}

fn give_me(line: &str) -> (Colour, Vec<Child>) {
    let r = MyRule::parse(Rule::Regulation, &line)
        .unwrap()
        .next()
        .unwrap();

    let mut parts = r.into_inner();
    let lhs = parts.next().unwrap();
    let rhs = parts.next().unwrap();

    let big = lhs.into_inner().next().unwrap().as_str().to_string();
    // println!("{}", big);

    let mut smalls = vec![];
    let smalls_iter = rhs.into_inner();

    for small in smalls_iter {
        let mut gr = small.into_inner();
        if let Some(qty_) = gr.next() {
            let qty = str::parse::<Qty>(&qty_.as_str().to_string()).expect("cannot parse qty");
            let bag_name = gr.next().unwrap().as_str().to_string();
            smalls.push((qty, bag_name));
        }
    }
    // println!("{:?}", smalls);

    (big, smalls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let (colour, children) =
            give_me("light silver bags contain 4 dim maroon bags, 1 mirrored teal bag.");
        assert_eq!(colour, "light silver".to_string());
        assert_eq!(children, vec![(4, "dim maroon".into())]);
    }

    #[test]
    fn test2() {
        let mut table: HashMap<Colour, Qty> = HashMap::new();
        table.insert("violet".into(), 0);

        let mut to_find: HashMap<Colour, Vec<Child>> = HashMap::new();
        to_find.insert("blue".into(), vec![(2, "violet".into())]);
        to_find.insert("green".into(), vec![(2, "blue".into())]);
        to_find.insert("yellow".into(), vec![(2, "green".into())]);
        to_find.insert("orange".into(), vec![(2, "yellow".into())]);
        to_find.insert("red".into(), vec![(2, "orange".into())]);
        to_find.insert("gold".into(), vec![(2, "red".into())]);

        let count = f("gold", &mut table, &mut to_find);
        assert_eq!(count, 126);
    }
}
