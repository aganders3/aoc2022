use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct Computer {
    instructions: VecDeque<Instruction>,
    i: Option<Instruction>,
    i_ctr: usize,
    cycle: isize,
    x: isize,
}

impl Computer {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        Computer {
            instructions, 
            i: None,
            i_ctr: 0,
            cycle: 0,
            x: 1
        }
    }

    fn step(&mut self) -> bool {
        if self.i_ctr == 0 {
            self.process_instruction();
        }

        if self.i_ctr > 0 {
            self.i_ctr -= 1;
            self.cycle += 1;
            return true
        }
        false
    }

    fn process_instruction(&mut self) {
        // process the current instruction
        match self.i {
            Some(Instruction::AddX(v)) => self.x += v,
            _ => (),
        }

        // grab the next instruction
        self.i = self.instructions.pop_front();

        // reset the instruction counter
        match &self.i {
            Some(inst) => self.i_ctr = inst.cycles(),
            _ => (),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    NoOp,
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::AddX(_) => 2,
            Self::NoOp => 1,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" { return Ok(Instruction::NoOp); }

        match s.split_once(' ') {
            Some(("addx", v)) => Ok(Instruction::AddX(v.parse().map_err(|_| "not a valid value")?)),
            _ => Err("wtf - not a recognized command".to_string()),
        }
    }
}

fn part_1(s: &str) -> isize {
    let instructions = s
        .lines()
        .map(|x| x.parse::<Instruction>().expect("failed to parse an instruction"))
        .collect::<VecDeque<_>>();
    let mut c = Computer::new(instructions);

    let mut interesting = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut result = 0;
    while c.step() && !interesting.is_empty() {
        if interesting.remove(&c.cycle) {
            result += dbg!(&c.cycle * &c.x);
        }
    }

    result
}

const WIDTH: isize = 40;

fn part_2(s: &str) {
    let instructions = s
        .lines()
        .map(|x| x.parse::<Instruction>().expect("failed to parse an instruction"))
        .collect::<VecDeque<_>>();
    let mut c = Computer::new(instructions);

    while c.step() {
        let h = &c.cycle % WIDTH;
        if h == 1 { print!("Cycle {:4} -> ", &c.cycle); }
        // cycles are 1-indexed, but pixels are 0-indexed
        if (&c.x - 1..=&c.x + 1).contains(&(h - 1)) {
            // print!("\x1b[93m#\x1b[0m");
            // make it a bit more visible
            print!("\x1b[0;0;100m#\x1b[0m");
        } else {
            print!(".");
        }
        if h == 0 { println!(" <- Cycle {:4}", &c.cycle); }
    }
}


fn main() {
    assert!(dbg!(part_1(include_str!("test.input.txt"))) == 13140);
    println!("Part 1: {}", part_1(include_str!("input.txt")));

    println!("");
    println!("Part 2: test input");
    part_2(include_str!("test.input.txt"));
    println!("");
    println!("Part 2: real input");
    part_2(include_str!("input.txt"));
}
