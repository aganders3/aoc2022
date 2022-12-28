use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn calculate(&self, m1: isize, m2: isize) -> isize {
        match self {
            Op::Add => m1 + m2,
            Op::Sub => m1 - m2,
            Op::Div => m1 / m2,
            Op::Mul => m1 * m2,
        }
    }
}

impl FromStr for Op {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "/" => Ok(Op::Div),
            "*" => Ok(Op::Mul),
            _ => Err(ParseMonkeyError),
        }
    }
}

#[derive(Clone, Debug)]
enum Monkey {
    Done(isize),
    Pending(String, String, Op),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMonkeyError;

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<isize>() {
            Ok(Monkey::Done(num))
        } else {
            let mut tokens = s.split(' ');
            let m1 = tokens.next().unwrap().to_string();
            let op = tokens.next().unwrap().parse().unwrap();
            let m2 = tokens.next().unwrap().to_string();
            Ok(Monkey::Pending(m1, m2, op))
        }
    }
}

fn parse(s: &str) -> HashMap<String, Monkey> {
    HashMap::from_iter(s.lines().map(|line| {
        let (monkey, op) = line.split_once(": ").unwrap();
        (monkey.to_string(), op.parse::<Monkey>().unwrap())
    }))
}

fn part_1(mut jobs: HashMap<String, Monkey>) -> isize {
    let mut pending = VecDeque::from(["root".to_string()]);
    let mut pending_set = HashSet::from(["root".to_string()]);

    while !pending.is_empty() {
        let job = pending.pop_front().expect("we just checked!");
        pending_set.remove(&job);
        if let Monkey::Pending(m1, m2, op) = jobs.get(&job).expect("job not found!") {
            match (
                jobs.get(m1).expect("monkey 1 not found!"),
                jobs.get(m2).expect("monkey 2 not found!"),
            ) {
                (Monkey::Done(num1), Monkey::Done(num2)) => {
                    jobs.insert(job.to_string(), Monkey::Done(op.calculate(*num1, *num2)));
                    if job == "root" {
                        break;
                    }
                }
                (Monkey::Pending(_, _, _), Monkey::Pending(_, _, _)) => {
                    if pending_set.insert(m1.to_string()) {
                        pending.push_back(m1.to_string());
                    }
                    if pending_set.insert(m2.to_string()) {
                        pending.push_back(m2.to_string());
                    }
                    pending.push_back(job.to_string());
                    pending_set.insert(job.to_string());
                }
                (Monkey::Pending(_, _, _), _) => {
                    if pending_set.insert(m1.to_string()) {
                        pending.push_back(m1.to_string());
                    }
                    pending.push_back(job.to_string());
                    pending_set.insert(job.to_string());
                }
                (_, Monkey::Pending(_, _, _)) => {
                    if pending_set.insert(m2.to_string()) {
                        pending.push_back(m2.to_string());
                    }
                    pending.push_back(job.to_string());
                    pending_set.insert(job.to_string());
                }
            }
        }
    }

    if let Monkey::Done(res) = dbg!(jobs.get("root").unwrap()) {
        *res
    } else {
        0
    }
}

fn main() {
    let jobs = parse(include_str!("test.input.txt"));
    assert_eq!(part_1(jobs), 152);

    let jobs = parse(include_str!("input.txt"));
    println!("Part 1 {}", part_1(jobs));
}
