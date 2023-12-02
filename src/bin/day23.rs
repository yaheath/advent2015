use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::vm_shell::{CPU, VM, VMShell, InstructionResult};
//use advent_lib::vm_display::{InstructionDisplay, Formatter};
//use advent_lib::vm_debugger::Debugger;

#[derive(Debug, Clone)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i64),
    Jie(char, i64),
    Jio(char, i64),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let opcode = iter.next().ok_or(())?;
        let arg1 = iter.next().ok_or(())?;
        let arg1c = arg1.chars().next().ok_or(())?;
        let arg2 = iter.next().map(|v| v.parse::<i64>().unwrap());
        match opcode {
            "hlf" => Ok(Instruction::Hlf(arg1c)),
            "tpl" => Ok(Instruction::Tpl(arg1c)),
            "inc" => Ok(Instruction::Inc(arg1c)),
            "jmp" => Ok(Instruction::Jmp(arg1.parse::<i64>().unwrap())),
            "jie" => Ok(Instruction::Jie(arg1c, arg2.unwrap())),
            "jio" => Ok(Instruction::Jio(arg1c, arg2.unwrap())),
            _ => Err(()),
        }
    }
}

struct JanesCPU {}

impl CPU<char, u64, Instruction> for JanesCPU {
    fn execute_instruction(&self, vm: &mut VM<char, u64, Instruction>, i: &Instruction) -> InstructionResult {
        match i {
            Instruction::Hlf(r) => {
                let n = vm.get_reg(*r) / 2;
                vm.set_reg(*r, n);
            },
            Instruction::Tpl(r) => {
                let n = vm.get_reg(*r) * 3;
                vm.set_reg(*r, n);
            },
            Instruction::Inc(r) => {
                let n = vm.get_reg(*r) + 1;
                vm.set_reg(*r, n);
            },
            Instruction::Jmp(n) if *n < 0 => {
                return InstructionResult::JumpBck(n.abs() as usize);
            },
            Instruction::Jmp(n) => {
                return InstructionResult::JumpFwd(*n as usize);
            },
            Instruction::Jie(r, n) => {
                let v = vm.get_reg(*r);
                if v & 1 == 0 {
                    if *n < 0 {
                        return InstructionResult::JumpBck(n.abs() as usize);
                    }
                    else {
                        return InstructionResult::JumpFwd(*n as usize);
                    }
                }
            }
            Instruction::Jio(r, n) => {
                let v = vm.get_reg(*r);
                if v == 1 {
                    if *n < 0 {
                        return InstructionResult::JumpBck(n.abs() as usize);
                    }
                    else {
                        return InstructionResult::JumpFwd(*n as usize);
                    }
                }
            },
        }
        InstructionResult::Ok
    }
}

struct JanesVM {
    cpu: JanesCPU,
    shell: VMShell<char, u64, Instruction>,
}

impl JanesVM {
    fn new(program: &Vec<Instruction>, a: u64) -> Self {
        let cpu = JanesCPU{};
        let mut shell = VMShell::new(program.clone(), 0);
        shell.vm.set_reg('a', a);
        Self { cpu, shell }
    }
    fn run(&mut self) {
        self.shell.run(&self.cpu);
    }
}

fn part1(input: &Vec<Instruction>) -> u64 {
    let mut vm = JanesVM::new(input, 0);
    vm.run();
    vm.shell.vm.get_reg('b')
}

fn part2(input: &Vec<Instruction>) -> u64 {
    let mut vm = JanesVM::new(input, 1);
    vm.run();
    vm.shell.vm.get_reg('b')
}

fn main() {
    let input: Vec<Instruction> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<Instruction> = test_input("inc a\njio a, +2\ntpl a\ninc a\n");
        let mut vm = JanesVM::new(&input);
        vm.run();
        assert_eq!(vm.shell.vm.get_reg('a'), 2);
        //assert_eq!(part2(&input), 0);
    }
}
