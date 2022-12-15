use std::collections::HashSet;
use std::convert::TryFrom;

fn parse_input(s: &str) -> HashSet<(usize, usize)> {
    let mut rock: HashSet<(usize, usize)> = HashSet::new();

    s.lines().for_each(|line| {
        line.split(" -> ")
            .collect::<Vec<&str>>()
            .windows(2)
            .map(|x| <&[&str; 2]>::try_from(x).expect("line may be too short?"))
            .for_each(|&[s, e]| {
                // unwrap all the things
                let (x_start, y_start) = s.split_once(',').unwrap();
                let x_start: usize = x_start.parse().unwrap();
                let y_start: usize = y_start.parse().unwrap();

                let (x_end, y_end) = e.split_once(',').unwrap();
                let x_end: usize = x_end.parse().unwrap();
                let y_end: usize = y_end.parse().unwrap();

                let (x_min, x_max) = (x_start.min(x_end), x_start.max(x_end));
                let (y_min, y_max) = (y_start.min(y_end), y_start.max(y_end));

                assert!(x_min == x_max || y_min == y_max);

                for x in x_min..=x_max {
                    for y in y_min..=y_max {
                        rock.insert((x, y));
                    }
                }
            });
    });

    rock
}

fn so_are_the_days_of_our_lives(
    cur: &(usize, usize),
    rock: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    if !rock.contains(&(cur.0, cur.1 + 1)) {
        // straight down
        (cur.0, cur.1 + 1)
    } else if !rock.contains(&(cur.0 - 1, cur.1 + 1)) {
        // else to the left
        (cur.0 - 1, cur.1 + 1)
    } else if !rock.contains(&(cur.0 + 1, cur.1 + 1)) {
        // else to the right
        (cur.0 + 1, cur.1 + 1)
    } else {
        *cur
    }
}

const SOURCE: (usize, usize) = (500, 0);

fn part_1(s: &str) -> usize {
    let mut rock = parse_input(s);
    let max_rock = dbg!(rock.iter().max_by_key(|(_x, y)| y).expect("no rocks").1);

    let mut sand_at_rest = 0;
    let mut cur = SOURCE;
    let mut prev = cur;
    loop {
        // let the sand fall
        cur = so_are_the_days_of_our_lives(&cur, &rock);

        // falling into the ether
        if cur.1 >= max_rock {
            break;
        }

        // come to rest
        if cur == prev {
            // sand might as well be a rock at this point
            rock.insert(cur);
            sand_at_rest += 1;
            // respawn
            cur = SOURCE;
        }
        prev = cur;
    }

    sand_at_rest
}

fn part_2(s: &str) -> usize {
    let mut rock = parse_input(s);
    let floor = dbg!(2 + rock.iter().max_by_key(|(_x, y)| y).expect("no rocks").1);

    let mut sand_at_rest = 0;
    let mut cur = SOURCE;
    let mut prev = cur;
    loop {
        // let the sand fall
        cur = so_are_the_days_of_our_lives(&cur, &rock);

        // come to rest
        if cur == prev || cur.1 == floor - 1 {
            // sand might as well be a rock at this point
            rock.insert(cur);
            sand_at_rest += 1;
            // full up!
            if cur == SOURCE {
                break;
            }
            // respawn
            cur = SOURCE;
        }
        prev = cur;
    }
    sand_at_rest
}

fn main() {
    assert!(dbg!(part_1(include_str!("test.input.txt"))) == 24);
    let part_1_answer = part_1(include_str!("input.txt"));
    println!("Part 1: {part_1_answer}");

    assert!(dbg!(part_2(include_str!("test.input.txt"))) == 93);
    let part_2_answer = part_2(include_str!("input.txt"));
    println!("Part 2: {part_2_answer}");
}
