use std::str::FromStr;

#[derive(Debug)]
enum Command {
    ListDir,
    ChangeDir(Option<String>),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = s.strip_prefix("$ ").ok_or("not a command")?;
        match cmd.split_once(' ') {
            Some((_, "..")) => Ok(Command::ChangeDir(None)),
            Some((_, dir)) => Ok(Command::ChangeDir(Some(dir.to_string()))),
            None => Ok(Command::ListDir),
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((size, name)) => Ok(File {
                size: size.parse().map_err(|_| "bad file size")?,
                name: name.to_string(),
            }),
            None => Err("not a file".to_string()),
        }
    }
}

fn main() {
    assert!(part_1(include_str!("test.input.txt")) == 95437);
    println!("Part 1: {}", part_1(include_str!("input.txt")));

    assert!(part_2(include_str!("test.input.txt")) == 24933642);
    println!("Part 2: {}", part_2(include_str!("input.txt")));
}

fn part_1(input: &str) -> usize {
    let mut dir_stack = Vec::<String>::new();
    let mut dir_sizes = std::collections::HashMap::<String, usize>::new();
    let mut seen = std::collections::HashSet::<String>::new();
    input.lines().for_each(|line| {
        match line.parse::<Command>() {
            Ok(Command::ListDir) => (),
            Ok(Command::ChangeDir(None)) => {
                dir_stack.pop();
            }
            Ok(Command::ChangeDir(Some(dir))) => dir_stack.push(dir),
            _ => {
                // not a command, parse the listing as a (maybe) file
                if let Ok(file) = line.parse::<File>() {
                    if seen.insert(format!("{}/{}", dir_stack.join("/"), file.name)) {
                        let mut full_path = Vec::<String>::new();
                        for d in &dir_stack {
                            full_path.push(d.to_string());
                            dir_sizes
                                .entry(full_path.join("/").clone())
                                .and_modify(|e| *e += file.size)
                                .or_insert(file.size);
                        }
                    }
                }
            }
        }
    });
    dir_sizes
        .into_values()
        .fold(0, |acc, x| if x <= 100_000 { acc + x } else { acc })
}

fn part_2(input: &str) -> usize {
    let mut dir_stack = Vec::<String>::new();
    let mut dir_sizes = std::collections::HashMap::<String, usize>::new();
    let mut seen = std::collections::HashSet::<String>::new();
    input.lines().for_each(|line| {
        match line.parse::<Command>() {
            Ok(Command::ListDir) => (),
            Ok(Command::ChangeDir(None)) => {
                dir_stack.pop();
            }
            Ok(Command::ChangeDir(Some(dir))) => dir_stack.push(dir),
            _ => {
                // not a command, parse the listing as a (maybe) file
                if let Ok(file) = line.parse::<File>() {
                    if seen.insert(format!("{}/{}", dir_stack.join("/"), file.name)) {
                        let mut full_path = Vec::<String>::new();
                        for d in &dir_stack {
                            full_path.push(d.to_string());
                            dir_sizes
                                .entry(full_path.join("/").clone())
                                .and_modify(|e| *e += file.size)
                                .or_insert(file.size);
                        }
                    }
                }
            }
        }
    });
    // total FS size 70_000_000
    // min unused space 30_000_000
    let total_used = *dir_sizes.get(&"/".to_string()).unwrap();
    let to_delete = 30_000_000 - (70_000_000 - total_used);
    dir_sizes
        .into_values()
        .fold(usize::MAX, |acc, x| if x >= to_delete { acc.min(x) } else { acc })
}
