use std::fs::File;
use std::io::{BufRead, BufReader};

struct TreeCounter {
    jmp: usize,
    pos: usize,
    len: usize,
    count: i64,
}
impl TreeCounter {
    fn new(jmp: usize, len: usize) -> TreeCounter {
        TreeCounter {
            jmp,
            count: 0,
            pos: 0,
            len,
        }
    }
    fn inc(&mut self) {
        self.count += 1;
    }
    fn step(&mut self, row_of_trees: &str) {
        self.pos = (self.pos + self.jmp) % self.len;
        let c = row_of_trees.chars().nth(self.pos).unwrap();
        if c == '#' {
            self.inc();
        }
    }
    fn step_if_odd(&mut self, row_of_trees: &str, is_odd: bool) {
        if is_odd {
            self.step(&row_of_trees);
        }
    }
}

fn main() {
    let file = File::open("./src/input.txt").expect("cannot open file");
    let mut reader = BufReader::new(file).lines();

    let first_line = reader.next().unwrap().unwrap();
    let len = first_line.len();

    let mut counter1 = TreeCounter::new(1, len);
    let mut counter3 = TreeCounter::new(3, len);
    let mut counter5 = TreeCounter::new(5, len);
    let mut counter7 = TreeCounter::new(7, len);
    let mut counter1odd = TreeCounter::new(1, len);
    let mut is_odd = false;

    reader.map(Result::unwrap).for_each(|row_of_trees| {
        counter1.step(&row_of_trees);
        counter3.step(&row_of_trees);
        counter5.step(&row_of_trees);
        counter7.step(&row_of_trees);
        counter1odd.step_if_odd(&row_of_trees, is_odd);
        is_odd = !is_odd;
    });
    println!("No. of trees: {}", counter1.count);
    println!("No. of trees: {}", counter3.count);
    println!("No. of trees: {}", counter5.count);
    println!("No. of trees: {}", counter7.count);
    println!("No. of trees: {}", counter1odd.count);

    println!(
        "Product: {}",
        counter1.count * counter3.count * counter5.count * counter7.count * counter1odd.count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file = File::open("./src/test.txt").expect("cannot open file");
        let mut reader = BufReader::new(file).lines();

        let first_line = reader.next().unwrap().unwrap();
        let len = first_line.len();

        let mut trees1 = TreeCounter::new(1, len);
        let mut trees3 = TreeCounter::new(3, len);
        let mut trees5 = TreeCounter::new(5, len);
        let mut trees7 = TreeCounter::new(7, len);
        let mut trees1odd = TreeCounter::new(1, len);
        let mut is_odd = false;

        reader.map(Result::unwrap).for_each(|row_of_trees| {
            trees1.step(&row_of_trees);
            trees3.step(&row_of_trees);
            trees5.step(&row_of_trees);
            trees7.step(&row_of_trees);
            trees1odd.step_if_odd(&row_of_trees, is_odd);
            is_odd = !is_odd;
        });

        assert_eq!(trees1.count, 2);
        assert_eq!(trees3.count, 7);
        assert_eq!(trees5.count, 3);
        assert_eq!(trees7.count, 4);
        assert_eq!(trees1odd.count, 2);
    }
}
