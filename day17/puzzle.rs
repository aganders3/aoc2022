use std::collections::{BTreeSet, HashMap, HashSet};
use std::convert::TryInto;
use std::iter::Cloned;
use std::iter::Cycle;
use std::iter::FromIterator;

#[derive(Clone, Debug)]
enum RockShape {
    HorizontalLine,
    Plus,
    BackwardsL,
    VerticalLine,
    Square,
}

// TODO: try to use std::mem::MaybeUninit here?
#[derive(Clone, Debug)]
struct Rock {
    shape: RockShape,
    ll: (usize, usize),
}

impl Rock {
    fn rock_stream() -> Cycle<Cloned<std::slice::Iter<'static, RockShape>>> {
        [
            RockShape::HorizontalLine,
            RockShape::Plus,
            RockShape::BackwardsL,
            RockShape::VerticalLine,
            RockShape::Square,
        ]
        .iter()
        .cloned()
        .cycle()
    }

    fn step(&mut self, jet: &char, chamber: &mut Chamber) -> bool {
        // apply jet
        match jet {
            '<' => self.ll = (self.ll.0 - 1, self.ll.1),
            '>' => self.ll = (self.ll.0 + 1, self.ll.1),
            j => panic!("bad input jet {}", j),
        }

        if chamber.collision(&self) {
            // reverse the jet
            match jet {
                '<' => self.ll = (self.ll.0 + 1, self.ll.1),
                '>' => self.ll = (self.ll.0 - 1, self.ll.1),
                _ => unreachable!(),
            }
        };

        // fall down
        self.ll = (self.ll.0, self.ll.1 - 1);

        if chamber.collision(&self) {
            // reverse the fall
            self.ll = (self.ll.0, self.ll.1 + 1);
            // insert into chamber
            chamber.bricks.extend(self.solid_parts().iter());
            false
        } else {
            true
        }
    }

    fn solid_parts(&self) -> Vec<(usize, usize)> {
        match self.shape {
            RockShape::HorizontalLine => {
                vec![
                    (self.ll.0, self.ll.1),
                    (self.ll.0 + 1, self.ll.1),
                    (self.ll.0 + 2, self.ll.1),
                    (self.ll.0 + 3, self.ll.1),
                ]
            }
            RockShape::Plus => {
                vec![
                    (self.ll.0, self.ll.1 + 1),
                    (self.ll.0 + 1, self.ll.1),
                    (self.ll.0 + 1, self.ll.1 + 1),
                    (self.ll.0 + 1, self.ll.1 + 2),
                    (self.ll.0 + 2, self.ll.1 + 1),
                ]
            }
            RockShape::BackwardsL => {
                vec![
                    (self.ll.0, self.ll.1),
                    (self.ll.0 + 1, self.ll.1),
                    (self.ll.0 + 2, self.ll.1),
                    (self.ll.0 + 2, self.ll.1 + 1),
                    (self.ll.0 + 2, self.ll.1 + 2),
                ]
            }
            RockShape::VerticalLine => {
                vec![
                    (self.ll.0, self.ll.1),
                    (self.ll.0, self.ll.1 + 1),
                    (self.ll.0, self.ll.1 + 2),
                    (self.ll.0, self.ll.1 + 3),
                ]
            }
            RockShape::Square => {
                vec![
                    (self.ll.0, self.ll.1),
                    (self.ll.0, self.ll.1 + 1),
                    (self.ll.0 + 1, self.ll.1),
                    (self.ll.0 + 1, self.ll.1 + 1),
                ]
            }
        }
    }
}

// do we really have to keep track of the entire space?
// ah, only the bricks at and above the min of heights!
#[derive(Debug)]
struct Chamber {
    bricks: HashSet<(usize, usize)>,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            bricks: HashSet::new(),
        }
    }

    fn _print(&self) {
        for row in (0..=self.max()).rev() {
            for col in 0..=7 + 1 {
                if row == 0 {
                    print!("-");
                } else if col == 0 || col == 7 + 1 {
                    print!("|");
                } else if self.bricks.contains(&(col, row)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn collision(&self, rock: &Rock) -> bool {
        rock.solid_parts()
            .iter()
            .any(|x| self.bricks.contains(x) || !(1..=7).contains(&x.0) || x.1 == 0)
    }

    fn tops(&self) -> [usize; 7] {
        (1..=7)
            .map(|col| {
                self.bricks
                    .iter()
                    .filter_map(|rock| if rock.0 == col { Some(rock.1) } else { None })
                    .max()
                    .unwrap_or(0)
            })
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
    }

    fn max(&self) -> usize {
        self.bricks
            .iter()
            .max_by_key(|rock| rock.1)
            .unwrap_or(&(0, 0))
            .1
    }

    // prune below the min of heights of the columns
    fn prune(&mut self) -> BTreeSet<(usize, usize)> {
        let tops = self.tops();
        let min_top = tops.iter().min().unwrap();
        self.bricks.retain(|(_x, y)| y >= min_top);
        BTreeSet::from_iter(self.bricks.iter().map(|(x, y)| (*x, *y - (min_top - 1))))
    }
}

fn part_1(s: &str, n: usize) -> usize {
    let mut chamber = Chamber::new();

    let mut rock_shapes = Rock::rock_stream().take(n);
    let mut jets = s.chars().cycle();

    while let Some(shape) = rock_shapes.next() {
        let mut rock = Rock {
            shape,
            // spawn point for the rock
            ll: (3, chamber.max() + 4),
        };
        while rock.step(&jets.next().unwrap(), &mut chamber) {
        }
        chamber.prune();
    }
    dbg!(chamber.max())
}

fn part_2(s: &str, n: usize) -> usize {
    // hashmap is (rock_idx % 5, jet_idx, pruned top of the chamber): (rock_idx, height)
    let mut seen: HashMap<(usize, usize, BTreeSet<(usize, usize)>), (usize, usize)> = HashMap::new();
    let mut chamber = Chamber::new();

    let mut rock_shapes = Rock::rock_stream().enumerate().take(n);
    let mut jets = s.chars().enumerate().cycle();

    let mut period = None;
    let mut max = 0;
    let mut extra = 0;

    while let Some((rock_idx, shape)) = rock_shapes.next() {
        let mut rock = Rock {
            shape,
            // spawn point for the rock
            ll: (3, chamber.max() + 4),
        };

        // find the period
        loop {
            let (jet_idx, jet) = &jets.next().unwrap();
            if !rock.step(jet, &mut chamber) {
                // insert returns Some if an item is already in the HashMap
                if let Some((seen_rock_idx, seen_max)) =
                    seen.insert((rock_idx % 5, *jet_idx, chamber.prune()), (rock_idx, chamber.max()))
                {
                    period = Some(rock_idx - seen_rock_idx);
                    let periods_needed = (n - seen_rock_idx) / period.unwrap();
                    let height_per_period = chamber.max() - seen_max;
                    max = seen_max + periods_needed * height_per_period;
                    extra += (n - seen_rock_idx) % period.unwrap() - 1;  // subtract 1 because we're just over the period here
                }
                break;
            }
        }
        if period.is_some() { break; }
    }

    // run the extra few rocks
    let max_tmp = chamber.max();
    while let Some((_rock_idx, shape)) = rock_shapes.next() {
        let mut rock = Rock {
            shape,
            ll: (3, chamber.max() + 4),
        };
        while rock.step(&jets.next().unwrap().1, &mut chamber) {}
        extra -= 1;
        if extra == 0 { break; }
    }
    let max_extra = chamber.max() - max_tmp;

    dbg!(max + max_extra)
}

fn main() {
    assert!(part_1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022) == 3068);
    println!("Part 1: {}", part_1(include_str!("input.txt").trim(), 2022));

    assert!(part_2(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1_000_000_000_000) == 1514285714288);
    println!("Part 2: {}", part_2(include_str!("input.txt").trim(), 1_000_000_000_000));
}
