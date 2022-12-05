use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
struct Assignment(RangeInclusive<usize>);

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Assignment(start.parse()?..=end.parse()?))
    }
}

impl Assignment {
    fn overlaps(&self, other: &Assignment) -> bool {
        other.0.contains(&self.0.start())
            || other.0.contains(&self.0.end())
            || self.0.contains(&other.0.start())
            || self.0.contains(&other.0.end())
    }

    fn contains_entirely(&self, other: &Assignment) -> bool {
        self.0.start() <= other.0.start() && self.0.end() >= other.0.end()
    }
}

fn main() {
    // part 1
    let n = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| {
            (
                a.parse::<Assignment>().unwrap(),
                b.parse::<Assignment>().unwrap(),
            )
        })
        .filter(|(a, b)| a.contains_entirely(&b) || b.contains_entirely(&a))
        .count();
    println!("Part 1: {n}");

    // part 2
    let n = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| {
            (
                a.parse::<Assignment>().unwrap(),
                b.parse::<Assignment>().unwrap(),
            )
        })
        .filter(|(a, b)| a.overlaps(&b))
        .count();
    println!("Part 2: {n}");
}
