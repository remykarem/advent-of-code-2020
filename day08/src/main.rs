use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use Mnemonic::*;

type Operand = i32;
type Program = Vec<Instruction>;

#[derive(PartialEq, Debug, Clone)]
enum Mnemonic {
    Nop,
    Acc,
    Jmp,
}
type Instruction = (Mnemonic, Operand);

fn main() {
    let file = File::open("./src/input.txt").expect("cannot open input.txt");
    let reader = BufReader::new(file);

    let program: Program = reader
        .lines()
        .map(Result::unwrap)
        .map(VM::parse_line_into_instruction)
        .collect();

    let mut vm = VM::new(&program);
    let state = vm.exec_from(0);
    if state.is_err() {
        println!("Part 1: {}", vm.acc);
        vm.self_correct();
        println!("Part 2: {}", vm.acc);
    }
}

#[derive(Debug)]
struct VM<'a> {
    acc: i32,
    history: Vec<usize>,
    visited: HashSet<usize>, // actually this is kinda useless
    program: &'a [Instruction],
}
impl<'a> VM<'a> {
    fn parse_line_into_instruction(line: String) -> Instruction {
        let mut tokens = line.split_whitespace();
        let mnemonic = match tokens.next().expect("msg") {
            "nop" => Nop,
            "acc" => Acc,
            "jmp" => Jmp,
            _ => panic!("unsupported mnemonic"),
        };
        let arg =
            str::parse::<Operand>(tokens.next().expect("msg")).expect("cannot parse into i32");

        (mnemonic, arg)
    }
    fn new(program: &[Instruction]) -> VM {
        VM {
            acc: 0,
            history: vec![],
            visited: HashSet::new(),
            program,
        }
    }
    fn exec(&mut self, ip: usize) -> usize {
        self.history.push(ip);
        self.visited.insert(ip);
        let (mnemonic, arg) = &self.program[ip];

        match mnemonic {
            Nop => ip + 1,
            Acc => {
                self.acc += arg;
                ip + 1
            }
            Jmp => (ip as i32 + arg) as usize,
        }
    }
    fn undo(&mut self) -> usize {
        // returns next pointer as of current state
        if self.history.is_empty() {
            panic!("Illegal state");
        }

        let ip = self.history.pop().unwrap();
        self.visited.remove(&ip);

        let (mnemonic, arg) = &self.program[ip];
        if *mnemonic == Acc {
            self.acc -= arg;
        }

        ip
    }
    fn exec_from(&mut self, ip: usize) -> Result<(), usize> {
        let mut last_ip = ip;

        while last_ip != self.program.len() {
            let new_ip = self.exec(last_ip);
            if self.visited.contains(&new_ip) {
                // No undo will be made. Program halts just before it executes
                // an instruction that has previously been executed
                return Err(last_ip);
            }
            last_ip = new_ip;
        }
        Ok(())
    }
    fn undo_until_last_jmp_or_nop(&mut self) {
        let mut rollbacks = 0;

        for &ip in self.history.iter().rev() {
            let (mnemonic, _) = &self.program[ip];
            match mnemonic {
                Acc => rollbacks += 1,
                _ => break,
            }
        }

        for _ in 0..rollbacks {
            self.undo();
        }
    }
    fn self_correct(&mut self) {
        // Run this only after vm failed
        // DFS

        let mut res = Err(0); // dummy 0

        while res.is_err() {
            // Go to the last jmp/nop instruction
            self.undo_until_last_jmp_or_nop();

            // Pop this instruction
            let last_ip = self.undo();
            let (mnemonic, arg) = &self.program[last_ip];

            // Create a snapshot of current state
            let snapshot = (self.acc, self.history.clone());

            // Hack this instruction then manually move the ip
            let mut new_ip = last_ip;
            match mnemonic {
                Jmp => new_ip += 1,
                Nop => new_ip = (new_ip as i32 + arg) as usize,
                _ => panic!("illegal state"),
            }

            // Execute the edited instruction
            res = self.exec_from(new_ip);

            if res.is_err() {
                // Revert back to previous state
                self.acc = snapshot.0;
                self.history = snapshot.1;
            } else {
                // This is the change we want!
                self.history.insert(snapshot.1.len(), last_ip);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_execute_nop() {
        let program = [(Nop, 0)];
        let mut vm = VM::new(&program);
        assert_eq!(vm.exec(0), 1);
        assert_eq!(vm.acc, 0);
        assert_eq!(vm.history, vec![0]);
    }

    #[test]
    fn test_execute_acc() {
        let program = [(Acc, 10)];
        let mut vm = VM::new(&program);
        assert_eq!(vm.exec(0), 1);
        assert_eq!(vm.acc, 10);
        assert_eq!(vm.history, vec![0]);
    }

    #[test]
    fn test_execute_jmp() {
        let program = [(Jmp, 10)];
        let mut vm = VM::new(&program);
        assert_eq!(vm.exec(0), 10);
        assert_eq!(vm.acc, 0);
        assert_eq!(vm.history, vec![0]);
    }

    #[test]
    fn test_undo_with_len_1() {
        let program = [(Nop, 20)];
        let mut vm = VM::new(&program);
        vm.exec(0);
        vm.undo();
    }

    #[test]
    fn test_undo() {
        let program = [(Acc, 20), (Acc, 10)];
        let mut vm = VM::new(&program);
        vm.exec(0);
        vm.exec(1);
        let last_pointer = vm.undo();
        assert_eq!(last_pointer, 1);
        assert_eq!(vm.acc, 20);
    }

    #[test]
    fn test_exec_from() {
        let program = [(Acc, 20), (Acc, 10)];
        let mut vm = VM::new(&program);
        let res = vm.exec_from(0);
        assert!(res.is_ok());
        assert_eq!(vm.acc, 30);
    }

    #[test]
    fn test_exec_from_fail() {
        let program = [(Acc, 20), (Jmp, -1)];
        let mut vm = VM::new(&program);
        let res = vm.exec_from(0);
        assert!(res.is_err());
        assert!(res.err().unwrap() == 1);
        assert_eq!(vm.acc, 20);
        assert_eq!(vm.history, vec![0, 1]);
    }

    #[test]
    fn test_undo_until_last_jmp_or_nop() {
        let program = [(Acc, 20), (Jmp, 1), (Jmp, -1)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        assert_eq!(vm.acc, 20);
        assert_eq!(vm.history, vec![0, 1, 2]);

        vm.undo_until_last_jmp_or_nop();
        assert_eq!(vm.history, vec![0, 1, 2]);
    }

    #[test]
    fn test_undo_until_last_jmp_or_nop2() {
        let program = [(Jmp, 2), (Acc, 1), (Jmp, -1)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        assert_eq!(vm.acc, 1);
        assert_eq!(vm.history, vec![0, 2, 1]);

        vm.undo_until_last_jmp_or_nop();
        assert_eq!(vm.history, vec![0, 2]);
        assert_eq!(vm.acc, 0);
    }

    #[test]
    fn test_undo_until_last_jmp_or_nop3() {
        let program = [(Jmp, 3), (Acc, 1), (Acc, 1), (Jmp, -2)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        assert_eq!(vm.acc, 2);
        assert_eq!(vm.history, vec![0, 3, 1, 2]);

        vm.undo_until_last_jmp_or_nop();
        assert_eq!(vm.acc, 0);
        assert_eq!(vm.history, vec![0, 3]);
    }

    #[test]
    fn test_la() {
        // Problematic instruction.
        let program = [(Jmp, 3), (Acc, 1), (Acc, 1), (Jmp, -2)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());

        vm.self_correct();
        assert_eq!(vm.acc, 0);
    }

    #[test]
    fn test_la2() {
        // Problematic instruction.
        let program = [
            (Acc, 100),
            (Jmp, 2), // should be nop
            (Jmp, 2),
            (Jmp, -2),
        ];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        vm.self_correct();
        assert_eq!(vm.acc, 100);
    }

    #[test]
    fn test_la3() {
        // Problematic instruction.
        let program = [
            (Nop, 0), //
            (Acc, 1), //
            (Jmp, 4), //
            (Acc, 3), //
            (Jmp, -3),
            (Acc, -99),
            (Acc, 1),  //
            (Jmp, -4), //
            (Acc, 6),
        ];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        assert_eq!(vm.acc, 5);
        vm.self_correct();
        assert_eq!(vm.acc, 8);
    }

    #[test]
    #[should_panic]
    fn test_parse_line() {
        let line = "hello 10".into();
        VM::parse_line_into_instruction(line);
    }

    #[test]
    fn test_parse_line2() {
        let line = "jmp 10".into();
        let (instruction, arg) = VM::parse_line_into_instruction(line);
        assert_eq!(instruction, Jmp);
        assert_eq!(arg, 10);
    }

    #[test]
    fn test_parse_line_with_minus_sign() {
        let line = "acc -10".into();
        let (instruction, arg) = VM::parse_line_into_instruction(line);
        assert_eq!(instruction, Acc);
        assert_eq!(arg, -10);
    }

    #[test]
    fn test_parse_line_with_plus_sign() {
        let line = "acc +10".into();
        let (instruction, arg) = VM::parse_line_into_instruction(line);
        assert_eq!(instruction, Acc);
        assert_eq!(arg, 10);
    }

    #[test]
    fn parse_test() {
        let program: Program = vec![(Nop, 0), (Acc, 1)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_ok());
        assert_eq!(vm.acc, 1);
    }

    #[test]
    fn parse_test_should_just_accumulate() {
        let program: Program = vec![(Nop, 0), (Acc, 1_000_000), (Acc, 1_000_000)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_ok());
        assert_eq!(vm.acc, 2_000_000);
    }

    #[test]
    fn parse_test_jump() {
        let program: Program = vec![(Nop, 0), (Jmp, 2), (Acc, 1_000), (Acc, 1_000_000)];
        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_ok());
        assert_eq!(vm.acc, 1_000_000);
    }

    #[test]
    fn parse_test2() {
        let program: Program = vec![
            (Nop, 0),
            (Acc, 1),
            (Jmp, 4),
            (Acc, 3),
            (Jmp, -3),
            (Acc, -99),
            (Acc, 1),
            (Jmp, -4),
            (Acc, 6),
        ]
        ;

        let mut vm = VM::new(&program);
        assert!(vm.exec_from(0).is_err());
        assert_eq!(vm.acc, 5);
    }
}
