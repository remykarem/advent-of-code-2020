use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut vm = VM::new();
    vm.execute_program("./src/input.txt");
    println!("{}", vm.sum());
}

struct VM {
    mem: HashMap<u64, u64>,
    mask1: u64,
    mask2: u64,
}
impl VM {
    fn write(&mut self, addr: u64, mut value: u64) {
        value |= self.mask1;
        value &= self.mask2;
        self.mem.insert(addr, value);
    }
    fn new() -> VM {
        VM {
            mem: HashMap::new(),
            mask1: 0,
            mask2: 0,
        }
    }
    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
    fn execute_program(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open input file");
        let reader = BufReader::new(file);

        reader.lines().map(Result::unwrap).for_each(|line| {
            if let Some(raw_mask) = try_parse_mask(&line) {
                let (mask1, mask2) = create_masks(&raw_mask);
                self.mask1 = parse_binary_mask(&mask1);
                self.mask2 = parse_binary_mask(&mask2);
            } else {
                let (k, v) = try_parse_mem(&line).unwrap();
                self.write(k, v);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        "123".parse::<i32>().unwrap();
        let value = 0b00001011;
        let maske = 0b01111101;
        assert_eq!(value | maske, 0b11111111);
    }

    #[test]
    fn it_works2() {
        let z = i64::from_str_radix("01111101", 2).unwrap();
        assert_eq!(z, 0b01111101);
    }

    #[test]
    fn it_works3() {
        let raw_mask = "X1XXXX0X";
        let (mask1, mask2) = create_masks(raw_mask);
        assert_eq!(mask1, "01000010");
        assert_eq!(mask2, "11111101");
    }

    #[test]
    fn it_works4() {
        let mask1 = parse_binary_mask("01000010");
        let mask2 = parse_binary_mask("11111101");
        let mut value = 0b00001011;
        value |= mask1;
        assert_eq!(value, 0b01001011);
        value &= mask2;
        assert_eq!(value, 0b01001001);
    }


    #[test]
    fn it_works5() {
        let mask1 = 0b1000000;
        let mask2 = 0b0000010;
        let mut value = 0b1100101;
        value |= mask1;
        value &= mask2;
        assert_eq!(value, 0b1100101);
    }
    #[test]
    fn test_regx() {
        let re = Regex::new(r"^mask = ([10X]{36})$").unwrap();
        let text = "mask = 100110111X011X1X10110X11010001X11XX0";
        assert!(re.is_match(text));
        for cap in re.captures_iter(text) {
            println!("{}", &cap[1]);
        }
    }

    #[test]
    fn test_regx2() {
        let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        let text = "mem[134] = 109";
        assert!(re.is_match(text));
        for cap in re.captures_iter(text) {
            println!("mem: {} = {}", &cap[1], &cap[2]);
        }
    }

    #[test]
    fn test_vm() {
        let mut vm = VM::new();
        vm.execute_program("./src/test1.txt");
        assert_eq!(vm.mem.len(), 2);

        let (a, b) = create_masks("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(vm.mask1, parse_binary_mask(&a));
        assert_eq!(vm.mask2, parse_binary_mask(&b));

        assert_eq!(vm.mem[&7], 101);
        assert_eq!(vm.mem[&8], 64);

        assert_eq!(vm.sum(), 165);

    }
}

fn try_parse_mask(line: &str) -> Option<String> {
    let re = Regex::new(r"^mask = ([10X]{36})$").unwrap();

    if re.is_match(line) {
        let cap = re.captures_iter(line).next().unwrap();
        let mask = cap[1].to_string();
        Some(mask)
    } else {
        None
    }
}

fn try_parse_mem(line: &str) -> Option<(u64, u64)> {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    if re.is_match(line) {
        let cap = re.captures_iter(line).next().unwrap();
        let key = cap[1].parse::<u64>().unwrap();
        let value = cap[2].parse::<u64>().unwrap();
        Some((key, value))
    } else {
        None
    }
}

fn parse_binary_mask(mask: &str) -> u64 {
    u64::from_str_radix(mask, 2).unwrap()
}

fn create_masks(raw_mask: &str) -> (String, String) {
    // if raw_mask.len() != 26 {
    //     panic!("Incorrect length")
    // }

    let or_mask = raw_mask.replace('0', "1").replace('X', "0");
    let and_mask = raw_mask.replace('X', "1");
    (or_mask, and_mask)
}

fn altogether_now(raw_mask: &str, mut value: u64) -> u64 {
    let (mask1, mask2) = create_masks(raw_mask);
    let mask1 = parse_binary_mask(&mask1);
    let mask2 = parse_binary_mask(&mask2);

    // let mut value = 0b00001011;
    value |= mask1;
    value &= mask2;

    value
}
