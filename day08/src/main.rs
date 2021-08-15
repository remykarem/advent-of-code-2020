use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Mnemonic = String;
type Operand = i32;
type Instruction = (Mnemonic, Operand);

fn main() {
    let file = File::open("./src/input.txt").expect("cannot open input.txt");
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(Result::unwrap)
        .map(parse_line_into_instruction)
        .collect();

    let acc = execute_instructions(&instructions);
    println!("{:?}", acc);
}

fn parse_line_into_instruction(line: String) -> Instruction {
    let mut tokens = line.split_whitespace();
    let mnemonic = tokens.next().expect("msg");
    let arg = str::parse::<Operand>(tokens.next().expect("msg")).expect("cannot parse into i32");

    (mnemonic.into(), arg)
}

fn execute_instructions(instructions: &[Instruction]) -> i32 {
    let mut accumulator = 0;
    let mut ip: usize = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while ip < instructions.len() && !visited.contains(&ip) {
        visited.insert(ip);
        let (mnemonic, arg) = &instructions[ip];
        
        match mnemonic.as_str() {
            "nop" => ip += 1,
            "acc" => {
                accumulator += arg;
                ip += 1;
            }
            "jmp" => ip = (ip as i32 + arg) as usize,
            _ => panic!("unsupported mnemonic"),
        }
    }
    accumulator
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "hello 10".into();
        let (instruction, arg) = parse_line_into_instruction(line);
        assert_eq!(instruction, "hello");
        assert_eq!(arg, 10);
    }

    #[test]
    fn test_parse_line_with_minus_sign() {
        let line = "hello -10".into();
        let (instruction, arg) = parse_line_into_instruction(line);
        assert_eq!(instruction, "hello");
        assert_eq!(arg, -10);
    }

    #[test]
    fn test_parse_line_with_plus_sign() {
        let line = "hello +10".into();
        let (instruction, arg) = parse_line_into_instruction(line);
        assert_eq!(instruction, "hello");
        assert_eq!(arg, 10);
    }

    #[test]
    fn parse_test() {
        let instructions: Vec<Instruction> = [("nop".into(), 0), ("acc".into(), 1)].to_vec();
        let result = execute_instructions(&instructions);
        assert_eq!(result, 1);
    }

    #[test]
    fn parse_test_should_just_accumulate() {
        let instructions: Vec<Instruction> = [
            ("nop".into(), 0),
            ("acc".into(), 1_000_000),
            ("acc".into(), 1_000_000),
        ]
        .to_vec();
        let result = execute_instructions(&instructions);
        assert_eq!(result, 2_000_000);
    }

    #[test]
    fn parse_test_jump() {
        let instructions: Vec<Instruction> = [
            ("nop".into(), 0),
            ("jmp".into(), 2),
            ("acc".into(), 1_000),
            ("acc".into(), 1_000_000),
        ]
        .to_vec();
        let result = execute_instructions(&instructions);
        assert_eq!(result, 1_000_000);
    }

    #[test]
    fn parse_test2() {
        let instructions: Vec<Instruction> = [
            ("nop".into(), 0),
            ("acc".into(), 1),
            ("jmp".into(), 4),
            ("acc".into(), 3),
            ("jmp".into(), -3),
            ("acc".into(), -99),
            ("acc".into(), 1),
            ("jmp".into(), -4),
            ("acc".into(), 6),
        ]
        .to_vec();

        let result = execute_instructions(&instructions);
        assert_eq!(result, 5);
    }
}
