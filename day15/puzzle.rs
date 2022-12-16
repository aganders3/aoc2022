use std::collections::{HashMap, HashSet};
use std::ops::Sub;
use std::str::FromStr;

const CHARS_TO_KEEP: [char; 13] = [
    ',', ':', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Loc(isize, isize);

impl Sub for Loc {
    type Output = isize;

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn parse(s: &str) -> Vec<(Loc, Loc)> {
    s.lines()
        .map(|line| {
            line.chars()
                .filter(|x| CHARS_TO_KEEP.contains(x))
                .collect::<String>()
        })
        .map(|x| {
            let (s, b) = x.split_once(':').expect("wtf?");
            let (sx, sy) = s.split_once(',').expect("bad sensor");
            let s = Loc(isize::from_str(sx).unwrap(), isize::from_str(sy).unwrap());
            let (bx, by) = b.split_once(',').expect("bad beacon");
            let b = Loc(isize::from_str(bx).unwrap(), isize::from_str(by).unwrap());
            (s, b)
        })
        .collect::<Vec<_>>()
}

fn part_1(locs: Vec<(Loc, Loc)>, y_loc: isize) -> isize {
    let mut sensors: HashMap<Loc, (isize, Loc)> = HashMap::new();
    let mut beacons: HashSet<Loc> = HashSet::new();
    let mut no_beacons: HashSet<Loc> = HashSet::new();
    locs.into_iter().for_each(|(s, b)| {
        sensors.insert(s, (s - b, b));
        beacons.insert(b);
    });

    // this is still pretty slow but it works - only look at the row we care about
    sensors.iter().for_each(|(s, (r, _b))| {
        for x in 0..*r {
            let loc = Loc(s.0 - x as isize, y_loc);
            if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                no_beacons.insert(loc);
            }
            let loc = Loc(s.0 + x as isize, y_loc);
            if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                no_beacons.insert(loc);
            }
        }
    });

    // splat the sensor grids into the no_beacons set
    // this takes way too fuckin' long
    /*
    sensors.iter().for_each(|(s, (r, b))| {
        dbg!(r);
        for x in 0..=*r {
            for y in 0..=*r {
                let loc = Loc(s.0 - x as isize, s.1 - y as isize);
                if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                    no_beacons.insert(loc);
                }

                let loc = Loc(s.0 + x as isize, s.1 - y as isize);
                if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                    no_beacons.insert(loc);
                }

                let loc = Loc(s.0 - x as isize, s.1 + y as isize);
                if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                    no_beacons.insert(loc);
                }

                let loc = Loc(s.0 + x as isize, s.1 + y as isize);
                if loc - *s <= *r && !sensors.contains_key(&loc) && !beacons.contains(&loc) {
                    no_beacons.insert(loc);
                }
            }
        }
    });
    */

    dbg!(no_beacons.iter().filter(|Loc(_x, y)| *y == y_loc).count() as isize)
}

fn part_2(locs: Vec<(Loc, Loc)>, max_grid: isize) -> isize {
    let mut sensors: HashMap<Loc, (isize, Loc)> = HashMap::new();
    let mut beacons: HashSet<Loc> = HashSet::new();
    locs.into_iter().for_each(|(s, b)| {
        sensors.insert(s, (s - b, b));
        beacons.insert(b);
    });

    let mut x = 0;
    let mut y = 0;
    loop {
        let loc = Loc(x, y);
        let mut maybe_beacon = !sensors.contains_key(&loc) && !beacons.contains(&loc);

        let jump = sensors.iter().fold(1, |acc, (s, (r, _b))| {
            if loc - *s <= *r {
                // too close to this sensor
                maybe_beacon = false;
                let dx = s.0 - loc.0;
                let dy = s.1 - loc.1;
                acc.max((r - dy.abs()) + dx)
            } else {
                acc
            }
        });
        if maybe_beacon {
            break;
        }
        x += jump;

        if x > max_grid {
            x = 0;
            y += 1;
        }
    }

    dbg!(x * 4_000_000 + y)
}

fn main() {
    assert!(part_1(parse(include_str!("test.input.txt")), 10) == 26);
    let part_1_res = part_1(parse(include_str!("input.txt")), 2_000_000);
    println!("Part 1: {part_1_res}");
    assert!(part_2(parse(include_str!("test.input.txt")), 20) == 56000011);
    let part_2_res = part_2(parse(include_str!("input.txt")), 4_000_000);
    println!("Part 2: {part_2_res}");
}
