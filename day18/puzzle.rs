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

    dbg!(exposed_faces)
}

fn part_2(s: &str) -> usize {
    let cubes = s.lines().map(Cube::parse).collect::<HashSet<_>>();
    let mut outside: HashSet<Cube> = HashSet::new();

    let x = cubes.iter().filter_map(|c| Some(c.x));
    let min_x = x.clone().min().unwrap();
    let max_x = x.max().unwrap();

    let y = cubes.iter().filter_map(|c| Some(c.y));
    let min_y = y.clone().min().unwrap();
    let max_y = y.max().unwrap();

    let z = cubes.iter().filter_map(|c| Some(c.z));
    let min_z = z.clone().min().unwrap();
    let max_z = z.max().unwrap();

    // use a closure to capture the above values
    let in_bounds = |cube: &Cube| -> bool {
        (min_x - 1..=max_x + 1).contains(&cube.x)
        && (min_y - 1..=max_y + 1).contains(&cube.y)
        && (min_z - 1..=max_z + 1).contains(&cube.z)
    };

    let mut exposed_faces = 0;

    cubes.iter().for_each(|cube| {
        // neighbors are adjacent to the faces of each cube - this is what we're trying to count
        cube.neighbors().iter().for_each(|n| {
            if outside.contains(&n) {
                // we already know this face is exposed!
                exposed_faces += 1;
            } else if !cubes.contains(&n) {
                // BFS to try to get to outside
                let mut to_visit = VecDeque::from([vec![*n]]);
                let mut seen: HashSet<Cube> = HashSet::new();
                let mut exposed = false;
                while !to_visit.is_empty() {
                    let cur = to_visit.pop_front().expect("we just checked...");
                    for c in cur.last().unwrap().neighbors() {
                        // don't backtrack and only follow open space
                        if seen.contains(&c) || cubes.contains(&c) {
                            continue;
                        }

                        // known to be outside
                        if outside.contains(&c) || !in_bounds(&c) {
                            exposed_faces += 1;
                            outside.extend(cur.iter().cloned());
                            outside.insert(c);
                            exposed = true;
                            break;
                        }

                        seen.insert(c);
                        to_visit.push_back(
                            cur.iter().chain([&c]).cloned().collect()
                        );
                    }
                    if exposed { break; }
                }
            }
        });
    });

    dbg!(exposed_faces)
}

fn main() {
    assert_eq!(part_1(parse(include_str!("test.input.txt"))), 64);
    println!("Part 1: {}", part_1(parse(include_str!("input.txt"))));

    assert_eq!(part_2(parse(include_str!("test.input.txt"))), 58);
    println!("Part 2: {}", part_2(parse(include_str!("input.txt"))));
}
