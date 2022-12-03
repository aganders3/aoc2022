use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone, Debug)]
struct ParseRPSError;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum RPSOutcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl FromStr for RPSOutcome {
    type Err = ParseRPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RPSOutcome::Loss),
            "Y" => Ok(RPSOutcome::Draw),
            "Z" => Ok(RPSOutcome::Win),
            _ => Err(ParseRPSError),
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for RPSMove {
    type Err = ParseRPSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPSMove::Rock),
            "B" | "Y" => Ok(RPSMove::Paper),
            "C" | "Z" => Ok(RPSMove::Scissors),
            _ => Err(ParseRPSError),
        }
    }
}

fn get_score(them: RPSMove, us: RPSMove) -> u8 {
    let game_score = match (them, us) {
        // win
        (RPSMove::Rock, RPSMove::Paper)
        | (RPSMove::Paper, RPSMove::Scissors)
        | (RPSMove::Scissors, RPSMove::Rock) => RPSOutcome::Win,
        // lose
        (RPSMove::Rock, RPSMove::Scissors)
        | (RPSMove::Paper, RPSMove::Rock)
        | (RPSMove::Scissors, RPSMove::Paper) => RPSOutcome::Loss,
        // tie
        _ => RPSOutcome::Draw,
    };
    us as u8 + game_score as u8
}

fn get_move(them: RPSMove, outcome: RPSOutcome) -> RPSMove {
    match (them, outcome) {
        (them, RPSOutcome::Draw) => them,
        (RPSMove::Scissors, RPSOutcome::Win) | (RPSMove::Paper, RPSOutcome::Loss) => RPSMove::Rock,
        (RPSMove::Rock, RPSOutcome::Win) | (RPSMove::Scissors, RPSOutcome::Loss) => RPSMove::Paper,
        (RPSMove::Paper, RPSOutcome::Win) | (RPSMove::Rock, RPSOutcome::Loss) => RPSMove::Scissors,
    }
}

fn main() {
    // part 1
    let scores = read_lines("./input.txt").unwrap().map(|line| {
        let line = line.unwrap();
        let mut guide = line.split_whitespace();
        let them: RPSMove = guide
            .next()
            .expect("no first move")
            .parse()
            .expect("invalid first move");
        let us: RPSMove = guide
            .next()
            .expect("no second move")
            .parse()
            .expect("invalid second move");
        get_score(them, us) as u32
    });
    println!("Part 1: {}", scores.sum::<u32>());

    // part 2
    let scores = read_lines("./input.txt").unwrap().map(|line| {
        let line = line.unwrap();
        let mut guide = line.split_whitespace();
        let them: RPSMove = guide
            .next()
            .expect("no first move")
            .parse()
            .expect("invalid first move");
        let outcome: RPSOutcome = guide
            .next()
            .expect("no outcome")
            .parse()
            .expect("invalid outcome");
        get_score(them, get_move(them, outcome)) as u32
    });
    println!("Part 2: {}", scores.sum::<u32>());
}
