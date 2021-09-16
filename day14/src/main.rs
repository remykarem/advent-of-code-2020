use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut vm = VM2::new();
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
    use std::hash::Hash;

    use super::*;
    #[test]

    fn it_works() {
        "123".parse::<i32>().unwrap();
        let value = 0b00001011;
        let maske = 0b01111101;
        assert_eq!(value | maske, 0b01111111);
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
        assert_eq!(value, 0b1100101);
        value &= mask2;
        assert_eq!(value, 0b0000000);
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

    #[test]
    fn test_vm2() {
        let mut mask_counter = AddressIter {
            mask: 0,
            pos: vec![0, 2, 5],
            adder: 0,
            power: HashMap::new()
        };
        assert_eq!(mask_counter.find_sum(), 0);

        mask_counter.next(); // 0b001
        assert_eq!(mask_counter.find_sum(), 1 * 2u64.pow(0));

        mask_counter.next(); // 0b010
        assert_eq!(mask_counter.find_sum(), 1 * 2u64.pow(2));

        mask_counter.next(); // 0b011
        assert_eq!(mask_counter.find_sum(), 1 * 2u64.pow(0) + 1 * 2u64.pow(2));

        mask_counter.next(); // 0b100
        assert_eq!(mask_counter.find_sum(), 1 * 2u64.pow(5));

        mask_counter.next(); // 0b101
        assert_eq!(mask_counter.find_sum(), 1 * 2u64.pow(0) + 1 * 2u64.pow(5));

        mask_counter.next(); // 0b110
        mask_counter.next(); // 0b111
        assert_eq!(
            mask_counter.find_sum(),
            1 * 2u64.pow(0) + 1 * 2u64.pow(2) + 1 * 2u64.pow(5)
        );
    }

    #[test]
    fn test_mask() {
        let (vec, sum) = generate("XX01X");
        assert_eq!(vec, vec![0, 3, 4]);
        assert_eq!(sum, 2);
    }

    #[test]
    fn test_mask2() {
        let (vec, sum) = generate("XXXXX");
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_mask3() {
        let (vec, sum) = generate("11101");
        assert_eq!(vec, vec![]);
        assert_eq!(sum, 0b11101);
    }

    #[test]
    fn test_yo() {
        assert_eq!(address_plus_mask("0011", "XXXX"), "XXXX");
        assert_eq!(address_plus_mask("0010", "0000"), "0010");
        assert_eq!(address_plus_mask("0000", "1111"), "1111");
        assert_eq!(address_plus_mask("0010", "0010"), "0010");
        assert_eq!(address_plus_mask("1110", "001X"), "111X")
    }

    #[test]
    fn given_test_case1() {
        assert_eq!(
            address_plus_mask(
                "000000000000000000000000000000101010",
                "000000000000000000000000000000X1001X"
            ),
            "000000000000000000000000000000X1101X"
        )
    }

    #[test]
    fn given_test_case2() {
        assert_eq!(
            address_plus_mask(
                "000000000000000000000000000000011010",
                "00000000000000000000000000000000X0XX"
            ),
            "00000000000000000000000000000001X0XX"
        )
    }

    #[test]
    fn given_test_case3() {
        let mut iter = AddressIter::from_mask("00000000000000000000000000000001X0XX");
        assert_eq!(iter.adder, 2u64.pow(4));
        assert_eq!(iter.pos, vec![0, 1, 3]);
        assert_eq!(iter.find_sum(), 16);
        iter.inc();
        assert_eq!(iter.find_sum(), 17);
        iter.inc();
        assert_eq!(iter.find_sum(), 18);
        iter.inc();
        assert_eq!(iter.find_sum(), 19);
        iter.inc();
        assert_eq!(iter.find_sum(), 24);
    }

    #[test]
    fn given_test_case3_iter() {
        let mut iter = AddressIter::from_mask("00000000000000000000000000000001X0XX");
        assert_eq!(iter.next(), Some(16));
        assert_eq!(iter.next(), Some(17));
        assert_eq!(iter.next(), Some(18));
        assert_eq!(iter.next(), Some(19));
        assert_eq!(iter.next(), Some(24));
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(26));
        assert_eq!(iter.next(), Some(27));
    }
    #[test]
    fn given_test_case4_iter() {
        let mut iter = AddressIter::from_mask("000000000000000000000000000000X1101X");
        assert_eq!(iter.next(), Some(26));
        assert_eq!(iter.next(), Some(27));
        assert_eq!(iter.next(), Some(58));
        assert_eq!(iter.next(), Some(59));
    }
    #[test]
    fn given_test_case5_iter() {
        let mut vm = VM2::new();
        vm.execute_program("./src/test2.txt");
        println!("{:?}", vm.mem);
        assert_eq!(vm.sum(), 208);
    }
}

fn address_plus_mask(address: &str, mask: &str) -> String {

    let mut new_string = String::new();
    address.chars().zip(mask.chars()).for_each(|(a, m)| {
        if m == 'X' {
            new_string.push('X');
        } else if m == '1' {
            new_string.push('1');
        } else {
            new_string.push(a);
        }
    });

    new_string
}

fn generate(mask: &str) -> (Vec<u64>, u64) {
    let mut g: Vec<u64> = Vec::new();
    let mut sum = 0;

    mask.chars().rev().enumerate().for_each(|(i, c)| {
        if c == 'X' {
            g.push(i as u64);
        } else {
            sum += (c.to_digit(10).unwrap() as u64) * (2u64).pow(i as u32);
            // println!(
            //     "2^{}*{}={}",
            //     i,
            //     c,
            //     (c.to_digit(10).unwrap() as u64) * (2u64).pow(i as u32)
            // );
        }
    });

    (g, sum)
}

struct VM2 {
    mem: HashMap<u64, u64>,
    mask: String,
    address_iter: AddressIter,
}
impl VM2 {
    fn write(&mut self, addr: u64, value: u64) {
        self.mem.insert(addr, value);
    }
    fn execute_program(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open input file");
        let reader = BufReader::new(file);

        reader.lines().map(Result::unwrap).for_each(|line| {
            if let Some(raw_mask) = try_parse_mask(&line) {
                self.mask = raw_mask;
            } else {
                let (address, v) = try_parse_mem2(&line).unwrap();
                let the_mask = address_plus_mask(&address, &self.mask);
                self.address_iter = AddressIter::from_mask(&the_mask);

                while let Some(addr) = self.address_iter.next() {
                    self.write(addr, v);
                }
            }
        });
    }
    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
    fn new() -> Self {
        VM2 {
            mem: HashMap::new(),
            mask: String::new(),
            address_iter: AddressIter::from_mask("X"),
        }
    }
}

#[derive(Debug)]
struct AddressIter {
    mask: u64,
    pos: Vec<u64>,
    adder: u64,
    power: HashMap<u64,u64>,
}
impl AddressIter {
    fn find_sum(&self) -> u64 {
        self.pos
            .iter()
            .enumerate()
            .map(|(i, &p)| ((self.mask >> i) & 1) * 2u64.pow(p as u32))
            .sum::<u64>()
            + self.adder
    }
    fn inc(&mut self) {
        self.mask += 1;
    }
    fn from_mask(mask: &str) -> AddressIter {
        let (pos, adder) = generate(mask);
        AddressIter {
            mask: 0,
            pos,
            adder,
            power: HashMap::new(),
        }
    }
    fn new(pos: Vec<u64>, adder: u64) -> AddressIter {
        AddressIter {
            mask: 0,
            pos,
            adder,
            power: HashMap::new(),
        }
    }
    fn pow(&self, exp: u32) -> u64 {
        self.power.entry(exp).or_insert(2u64.pow(p as u32))
    }
}
impl Iterator for AddressIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask > 2u64.pow(self.pos.len() as u32) {
            return None;
        }

        let sum = self
            .pos
            .iter()
            .enumerate()
            .map(|(i, &p)| ((self.mask >> i) & 1) * 2u64.pow(p as u32))
            .sum::<u64>()
            + self.adder;

        self.mask += 1;

        Some(sum)
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
fn try_parse_mem2(line: &str) -> Option<(String, u64)> {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    if re.is_match(line) {
        let cap = re.captures_iter(line).next().unwrap();
        let key = cap[1].parse::<u64>().unwrap();
        let value = cap[2].parse::<u64>().unwrap();
        Some((format!("{:036b}", key), value))
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
