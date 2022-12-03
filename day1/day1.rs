use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elves = vec![0];
        for line in lines {
            if let Ok(cals) = line.unwrap().parse::<usize>() {
                *elves.last_mut().unwrap() += cals;
            } else {
                elves.push(0);
            }
        }
        elves.sort();
        println!("Part 1: {:?}", elves.last().unwrap());
        println!("Part 2: {:?}", elves.iter().rev().take(3).sum::<usize>());
    }
}

// from rust by example
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
