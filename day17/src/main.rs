use std::fmt::Debug;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Opcode {
    Adv = 0,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Opcode {
    fn from(value: isize) -> Opcode {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}

struct Instruction {
    opcode: Opcode,
    operand: isize,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.opcode, self.operand)
    }
}

#[derive(Debug)]
struct Device {
    pc: usize,
    a: isize,
    b: isize,
    c: isize,
    screen: Vec<isize>,
    memory: Vec<isize>,
}

impl Device {
    fn boot(registers: [isize; 3], memory: Vec<isize>) -> Self {
        Self {
            pc: 0,
            a: registers[0],
            b: registers[1],
            c: registers[2],
            screen: Vec::new(),
            memory,
        }
    }
    fn get_combo_operand(&self, operand: isize) -> isize {
        match operand {
            0 | 1 | 2 | 3 | 7 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }
    fn dv(&mut self, operand: isize) -> isize {
        let combo = self.get_combo_operand(operand) as u32;
        self.a / 2isize.pow(combo)
    }
    fn adv(&mut self, operand: isize) {
        self.a = self.dv(operand);
    }
    fn bxl(&mut self, operand: isize) {
        self.b ^= operand
    }
    fn bst(&mut self, operand: isize) {
        self.b = self.get_combo_operand(operand) % 8
    }
    fn bxc(&mut self, _ioperand: isize) {
        self.b ^= self.c
    }
    fn out(&mut self, operand: isize) {
        let result = self.get_combo_operand(operand) % 8;
        self.screen.push(result);
    }
    fn bdv(&mut self, operand: isize) {
        self.b = self.dv(operand);
    }
    fn cdv(&mut self, operand: isize) {
        self.c = self.dv(operand);
    }
    fn run(&mut self) {
        while self.pc < self.memory.len() {
            let value = self.memory[self.pc];
            let opcode = Opcode::from(value);
            let operand = self.memory[self.pc + 1];
            if opcode != Opcode::Jnz {
                self.execute(Instruction { opcode, operand });
            } else if self.a != 0 {
                self.pc = operand as usize;
                continue;
            }
            self.pc += 2
        }
    }
    fn execute(&mut self, instr: Instruction) {
        let Instruction { opcode, operand } = instr;
        match opcode {
            Opcode::Adv => self.adv(operand),
            Opcode::Bxl => self.bxl(operand),
            Opcode::Bst => self.bst(operand),
            Opcode::Jnz => {}
            Opcode::Bxc => self.bxc(operand),
            Opcode::Out => self.out(operand),
            Opcode::Bdv => self.bdv(operand),
            Opcode::Cdv => self.cdv(operand),
        }
    }
    fn display(&self) {
        let output = self
            .screen
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
        println!("{}", output);
    }
    fn dump_registers(&self) {
        println!("[a: {} b: {} c: {}]", self.a, self.b, self.c);
    }
}

fn parse(txt: &str, regex: &Regex) -> Vec<isize> {
    regex
        .captures_iter(txt)
        .map(|x| x[0].parse::<isize>().unwrap())
        .collect()
}

fn solve_part1(registers: &[isize], memory: &[isize]) {
    let mut device = Device::boot(registers.try_into().unwrap(), memory.to_owned());
    device.run();
    device.dump_registers();
    device.display();
}

fn main() {
    let (reg_info, mem_info) = include_str!("puzzle.txt").split_once("\n\n").unwrap();
    let re = Regex::new(r"(\d+)").unwrap();

    let registers: Vec<isize> = parse(reg_info, &re);
    let memory: Vec<isize> = parse(mem_info, &re);

    solve_part1(&registers, &memory)
}
