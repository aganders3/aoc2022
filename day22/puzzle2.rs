use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Face {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6 
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Loc {
    face: Face,
    row: isize,
    col: isize,
}

impl Loc {
    fn from_input_space(n: usize, i: usize, j: usize) -> Loc {
        // match syntax abuse?
        let face = match i {
            x if (0..n).contains(&x) => {
                match j {
                    y if (n..2*n).contains(&y) => Face::One,
                    y if (2*n..3*n).contains(&y) => Face::Two,
                    _ => unreachable!(),
                }
            },
            x if (n..2*n).contains(&x) => Face::Three,
            x if (2*n..3*n).contains(&x) => {
                match j {
                    y if (0..n).contains(&y) => Face::Four,
                    y if (n..2*n).contains(&y) => Face::Five,
                    _ => unreachable!(),
                }
            },
            x if (3*n..4*n).contains(&x) => Face::Six,
            _ => unreachable!(),
        };
        Loc { face, row: (i % n) as isize, col: (j % n) as isize }
    }

    fn to_input_space(&self, n: isize) -> (isize, isize) {
        match self.face {
            Face::One => (self.row, n + self.col),
            Face::Two => (self.row, 2*n + self.col),
            Face::Three => (n + self.row, n + self.col),
            Face::Four => (2*n + self.row, self.col),
            Face::Five => (2*n + self.row, n + self.col),
            Face::Six => (3*n + self.row, self.col),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Open,
    Solid,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: HashMap<Loc, Tile>,
    n: isize,
}

impl Map {
    fn get_start(&self) -> Loc {
        let col = self
            .tiles
            .iter()
            .filter_map(|(k, v)| {
                if k.face == Face::One && k.row == 0 && *v == Tile::Open {
                    Some(k.col)
                } else {
                    None
                }
            })
            .min()
            .unwrap();
        Loc { face: Face::One, row: 0, col }
    }
}

// direction is *within that face*
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

    fn move_ahead(&mut self, n: isize) {
        (0..n).for_each(|_| {
            let (nxt_loc, nxt_dir) = self.next_loc();
            match self.map.tiles.get(&nxt_loc).unwrap() {
                Tile::Open => {
                    self.loc = nxt_loc;
                    self.dir = nxt_dir;
                },
                Tile::Solid => (),
            };
        })
    }

    fn next_loc(&mut self) -> (Loc, Dir) {
        let next = match self.dir {
            Dir::Right => Loc { face: self.loc.face, row: self.loc.row, col: self.loc.col + 1 },
            Dir::Left => Loc { face: self.loc.face, row: self.loc.row, col: self.loc.col - 1 },
            Dir::Up => Loc { face: self.loc.face, row: self.loc.row - 1, col: self.loc.col },
            Dir::Down => Loc { face: self.loc.face, row: self.loc.row + 1, col: self.loc.col },
        };

        self.wrap_loc(next)
    }

    fn wrap_loc(&self, loc: Loc) -> (Loc, Dir) {
        let n = self.map.n;
        if (0..n).contains(&loc.row) && (0..n).contains(&loc.col) { return (loc, self.dir) }

        let (loc, dir) = if loc.col >= n {
            // moving off the face to the RIGHT
            match loc.face {
                Face::One => {
                    (
                        Loc { face: Face::Two, row: loc.row, col: 0},
                        Dir::Right,
                    )
                },
                Face::Two => {
                    (
                        Loc { face: Face::Five, row: n - 1 - loc.row, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Three => {
                    (
                        Loc { face: Face::Two, row: n - 1, col: loc.row },
                        Dir::Up,
                    )
                },
                Face::Four => {
                    (
                        Loc { face: Face::Five, row: loc.row, col: 0 },
                        Dir::Right,
                    )
                },
                Face::Five => {
                    (
                        Loc { face: Face::Two, row: n - 1 - loc.row, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Six => {
                    (
                        Loc { face: Face::Five, row: n - 1, col: loc.row },
                        Dir::Up,
                    )
                },
            }
        } else if loc.col < 0 {
            // moving off the face to the LEFT
            match loc.face {
                Face::One => {
                    (
                        Loc { face: Face::Four, row: n - 1 - loc.row, col: 0},
                        Dir::Right,
                    )
                },
                Face::Two => {
                    (
                        Loc { face: Face::One, row: loc.row, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Three => {
                    (
                        Loc { face: Face::Four, row: 0, col: loc.row },
                        Dir::Down,
                    )
                },
                Face::Four => {
                    (
                        Loc { face: Face::One, row: n - 1 - loc.row, col: 0 },
                        Dir::Right,
                    )
                },
                Face::Five => {
                    (
                        Loc { face: Face::Four, row: loc.row, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Six => {
                    (
                        Loc { face: Face::One, row: 0, col: loc.row },
                        Dir::Down,
                    )
                },
            }
        } else if loc.row < 0 {
            // moving off the face to the TOP
            match loc.face {
                Face::One => {
                    (
                        Loc { face: Face::Six, row: loc.col, col: 0},
                        Dir::Right,
                    )
                },
                Face::Two => {
                    (
                        Loc { face: Face::Six, row: n - 1, col: loc.col },
                        Dir::Up,
                    )
                },
                Face::Three => {
                    (
                        Loc { face: Face::One, row: n - 1, col: loc.col },
                        Dir::Up,
                    )
                },
                Face::Four => {
                    (
                        Loc { face: Face::Three, row: loc.col, col: 0 },
                        Dir::Right,
                    )
                },
                Face::Five => {
                    (
                        Loc { face: Face::Three, row: n - 1, col: loc.col },
                        Dir::Up,
                    )
                },
                Face::Six => {
                    (
                        Loc { face: Face::Four, row: n - 1, col: loc.col },
                        Dir::Up,
                    )
                },
            }
        } else if loc.row >= n {
            // moving off the face to the BOTTOM
            match loc.face {
                Face::One => {
                    (
                        Loc { face: Face::Three, row: 0, col: loc.col},
                        Dir::Down,
                    )
                },
                Face::Two => {
                    (
                        Loc { face: Face::Three, row: loc.col, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Three => {
                    (
                        Loc { face: Face::Five, row: 0, col: loc.col },
                        Dir::Down,
                    )
                },
                Face::Four => {
                    (
                        Loc { face: Face::Six, row: 0, col: loc.col },
                        Dir::Down,
                    )
                },
                Face::Five => {
                    (
                        Loc { face: Face::Six, row: loc.col, col: n - 1 },
                        Dir::Left,
                    )
                },
                Face::Six => {
                    (
                        Loc { face: Face::Two, row: 0, col: loc.col },
                        Dir::Down,
                    )
                },
            }
        } else {
            unreachable!();
        };

        (loc, dir)
    }
}

fn parse(s: &str, n: usize) -> (Map, Vec<String>) {
    let tiles = HashMap::from_iter(s.lines().take_while(|x| *x != "\n").enumerate().flat_map(
        |(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '.' => Some((Loc::from_input_space(n, i, j), Tile::Open)),
                '#' => Some((Loc::from_input_space(n, i, j), Tile::Solid)),
                _ => None,
            })
        },
    ));

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
            n: n as isize,
        },
        instructions,
    )
}

fn part_2(map: Map, instructions: Vec<String>) -> isize {
    let mut pc = PC {
        loc: map.get_start(),
        dir: Dir::Right,
        map: &map,
    };

    for i in instructions.iter() {
        if i == "R" {
            pc.turn_right();
        } else if i == "L" {
            pc.turn_left();
        } else {
            pc.move_ahead(i.parse().unwrap());
        }
    }

    let (row, col) = pc.loc.to_input_space(map.n);
    dbg!(1000 * (row + 1) + 4 * (col + 1) + pc.dir as isize)
}

fn main() {
    // let (map, instructions) = parse(include_str!("test.input.txt"), 4);
    // assert_eq!(part_2(map, instructions), 5031);

    let (map, instructions) = parse(include_str!("input.txt"), 50);
    println!("Part 2: {}", part_2(map, instructions));
}
