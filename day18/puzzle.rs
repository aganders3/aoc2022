use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn parse(s: &str) -> Self {
        let mut coords = s.split(',');
        Cube {
            x: coords
                .next()
                .expect("bad input")
                .parse()
                .expect("bad input"),
            y: coords
                .next()
                .expect("bad input")
                .parse()
                .expect("bad input"),
            z: coords
                .next()
                .expect("bad input")
                .parse()
                .expect("bad input"),
        }
    }

    fn is_touching(&self, other: &Cube) -> bool {
        // manhattan distance
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() == 1
    }

    fn neighbors(&self) -> [Cube; 6] {
        (-1..=1)
            .map(move |x| {
                (-1..=1).map(move |y| {
                    (-1..=1).map(move |z| Cube {
                        x: self.x + x,
                        y: self.y + y,
                        z: self.z + z,
                    })
                })
            })
            .flatten()
            .flatten()
            .filter(|c| c.is_touching(&self)) // extra work...
            .collect::<Vec<_>>()
            .try_into()
            .expect("wtf")
    }
}

fn parse(s: &str) -> &str {
    s
}

fn part_1(s: &str) -> usize {
    let cubes = s.lines().map(Cube::parse).collect::<Vec<_>>();
    let mut exposed_faces = cubes.len() * 6;

    // nested loops - super simple stuff
    for c1 in &cubes {
        for c2 in &cubes {
            if c1.is_touching(c2) {
                exposed_faces -= 1;
            }
        }
    }

    exposed_faces
}


fn main() {
    assert_eq!(part_1(parse(include_str!("test.input.txt"))), 64);
    println!("Part 1: {}", part_1(parse(include_str!("input.txt"))));
}
