use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Loc {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Open,
    Solid,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: HashMap<Loc, Tile>,
    row_ranges: Vec<RangeInclusive<usize>>,
    column_ranges: Vec<RangeInclusive<usize>>,
}

impl Map {
    fn get_start(&self) -> Loc {
        let col = self
            .tiles
            .iter()
            .filter_map(|(k, v)| {
                if k.row == 0 && *v == Tile::Open {
                    Some(k.col)
                } else {
                    None
                }
            })
            .min()
            .unwrap();
        Loc { row: 0, col }
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Right = 0,
    Up = 3,
    Left = 2,
    Down = 1,
}

#[derive(Copy, Clone, Debug)]
struct PC<'a> {
    loc: Loc,
    dir: Dir,
    map: &'a Map,
}

impl<'a> PC<'a> {
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    // this is terrible
    fn move_ahead(&mut self, n: usize) {
        for _ in 1..=n {
            let nxt_loc = match self.dir {
                Dir::Right => {
                    // wrapping add around
                    let nxt_col =
                        if !self.map.row_ranges[self.loc.row].contains(&(self.loc.col + 1)) {
                            *self.map.row_ranges[self.loc.row].start()
                        } else {
                            self.loc.col + 1
                        };
                    Loc {
                        row: self.loc.row,
                        col: nxt_col,
                    }
                }
                Dir::Left => {
                    // wrapping add around
                    let nxt_col =
                        if !self.map.row_ranges[self.loc.row].contains(&(self.loc.col - 1)) {
                            *self.map.row_ranges[self.loc.row].end()
                        } else {
                            self.loc.col - 1
                        };
                    Loc {
                        row: self.loc.row,
                        col: nxt_col,
                    }
                }
                Dir::Up => {
                    // wrapping add around
                    let nxt_row =
                        if !self.map.column_ranges[self.loc.col].contains(&(self.loc.row - 1)) {
                            *self.map.column_ranges[self.loc.col].end()
                        } else {
                            self.loc.row - 1
                        };
                    Loc {
                        row: nxt_row,
                        col: self.loc.col,
                    }
                }
                Dir::Down => {
                    // wrapping add around
                    let nxt_row =
                        if !self.map.column_ranges[self.loc.col].contains(&(self.loc.row + 1)) {
                            *self.map.column_ranges[self.loc.col].start()
                        } else {
                            self.loc.row + 1
                        };
                    Loc {
                        row: nxt_row,
                        col: self.loc.col,
                    }
                }
            };
            if *self.map.tiles.get(&nxt_loc).unwrap() == Tile::Open {
                self.loc = nxt_loc
            };
        }
    }
}

fn parse(s: &str) -> (Map, Vec<String>) {
    let tiles = HashMap::from_iter(s.lines().take_while(|x| *x != "\n").enumerate().flat_map(
        |(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '.' => Some((Loc { row: i, col: j }, Tile::Open)),
                '#' => Some((Loc { row: i, col: j }, Tile::Solid)),
                _ => None,
            })
        },
    ));

    let rows = 1 + tiles.iter().max_by_key(|(k, _)| k.row).unwrap().0.row;
    let cols = 1 + tiles.iter().max_by_key(|(k, _)| k.col).unwrap().0.col;

    let row_ranges = (0..rows)
        .map(|row| {
            let cols_in_row =
                tiles
                    .iter()
                    .filter_map(|(k, _)| if k.row == row { Some(k.col) } else { None });
            cols_in_row.clone().min().unwrap()..=cols_in_row.max().unwrap()
        })
        .collect();

    let column_ranges = (0..cols)
        .map(|col| {
            let rows_in_col =
                tiles
                    .iter()
                    .filter_map(|(k, _)| if k.col == col { Some(k.row) } else { None });
            rows_in_col.clone().min().unwrap()..=rows_in_col.max().unwrap()
        })
        .collect();

    dbg!(&rows, &cols);

    let (_, instructions) = s
        .split_once("\n\n")
        .expect("instructions separated by blank line");
    let instructions = instructions.replace('R', " R ");
    let instructions = instructions.replace('L', " L ");
    let instructions = instructions
        .trim()
        .split(' ')
        .map(|x| x.to_string())
        .collect();

    (
        Map {
            tiles,
            row_ranges,
            column_ranges,
        },
        instructions,
    )
}

fn part_1(map: Map, instructions: Vec<String>) -> usize {
    let mut pc = PC {
        loc: map.get_start(),
        dir: Dir::Right,
        map: &map,
    };

    for i in instructions {
        if i == "R" {
            pc.turn_right();
        } else if i == "L" {
            pc.turn_left();
        } else {
            pc.move_ahead(i.parse().unwrap());
        }
    }

    dbg!(1000 * (pc.loc.row + 1) + 4 * (pc.loc.col + 1) + pc.dir as usize)
}

fn main() {
    let (map, instructions) = parse(include_str!("test.input.txt"));
    assert_eq!(part_1(map, instructions), 6032);

    let (map, instructions) = parse(include_str!("input.txt"));
    println!("Part 1: {}", part_1(map, instructions));
}
