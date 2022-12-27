use std::collections::{HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Blueprint {
    ore_bot_cost: usize,
    clay_bot_cost: usize,
    obsidian_bot_cost: (usize, usize),
    geode_bot_cost: (usize, usize),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    time_remaining: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

impl State {
    fn buy_bot(&self, r: &Resource, b: &Blueprint) -> Option<Self> {
        match r {
            Resource::Ore => {
                if b.ore_bot_cost <= self.ore {
                    Some(State {
                        time_remaining: self.time_remaining - 1,
                        ore: self.ore - b.ore_bot_cost + self.ore_bots,
                        clay: self.clay + self.clay_bots,
                        obsidian: self.obsidian + self.obsidian_bots,
                        geode: self.geode + self.geode_bots,
                        ore_bots: self.ore_bots + 1,
                        clay_bots: self.clay_bots,
                        obsidian_bots: self.obsidian_bots,
                        geode_bots: self.geode_bots,
                    })
                } else {
                    None
                }
            }
            Resource::Clay => {
                if b.clay_bot_cost <= self.ore {
                    Some(State {
                        time_remaining: self.time_remaining - 1,
                        ore: self.ore - b.clay_bot_cost + self.ore_bots,
                        clay: self.clay + self.clay_bots,
                        obsidian: self.obsidian + self.obsidian_bots,
                        geode: self.geode + self.geode_bots,
                        ore_bots: self.ore_bots,
                        clay_bots: self.clay_bots + 1,
                        obsidian_bots: self.obsidian_bots,
                        geode_bots: self.geode_bots,
                    })
                } else {
                    None
                }
            }
            Resource::Obsidian => {
                if b.obsidian_bot_cost.0 <= self.ore && b.obsidian_bot_cost.1 <= self.clay {
                    Some(State {
                        time_remaining: self.time_remaining - 1,
                        ore: self.ore - b.obsidian_bot_cost.0 + self.ore_bots,
                        clay: self.clay - b.obsidian_bot_cost.1 + self.clay_bots,
                        obsidian: self.obsidian + self.obsidian_bots,
                        geode: self.geode + self.geode_bots,
                        ore_bots: self.ore_bots,
                        clay_bots: self.clay_bots,
                        obsidian_bots: self.obsidian_bots + 1,
                        geode_bots: self.geode_bots,
                    })
                } else {
                    None
                }
            }
            Resource::Geode => {
                if b.geode_bot_cost.0 <= self.ore && b.geode_bot_cost.1 <= self.obsidian {
                    Some(State {
                        time_remaining: self.time_remaining - 1,
                        ore: self.ore - b.geode_bot_cost.0 + self.ore_bots,
                        clay: self.clay + self.clay_bots,
                        obsidian: self.obsidian - b.geode_bot_cost.1 + self.obsidian_bots,
                        geode: self.geode + self.geode_bots,
                        ore_bots: self.ore_bots,
                        clay_bots: self.clay_bots,
                        obsidian_bots: self.obsidian_bots,
                        geode_bots: self.geode_bots + 1,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn has_excess_capacity(&self, b: &Blueprint) -> bool {
        self.ore_bots
            > b.ore_bot_cost
                .max(b.clay_bot_cost)
                .max(b.obsidian_bot_cost.0)
                .max(b.geode_bot_cost.0)
            || self.clay_bots > b.obsidian_bot_cost.1
            || self.obsidian_bots > b.geode_bot_cost.1
    }
}

fn parse(s: &str) -> Vec<Blueprint> {
    s.lines()
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|words| Blueprint {
            ore_bot_cost: words[6].parse().unwrap(),
            clay_bot_cost: words[12].parse().unwrap(),
            obsidian_bot_cost: (words[18].parse().unwrap(), words[21].parse().unwrap()),
            geode_bot_cost: (words[27].parse().unwrap(), words[30].parse().unwrap()),
        })
        .collect()
}

fn get_max(b: &Blueprint, t: usize) -> usize {
    let init_state = State {
        time_remaining: t,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
    };

    let max_ore = b.ore_bot_cost
                .max(b.clay_bot_cost)
                .max(b.obsidian_bot_cost.0)
                .max(b.geode_bot_cost.0);
    let max_clay = b.obsidian_bot_cost.1;
    let max_obsidian = b.geode_bot_cost.1;

    let mut best = 0;
    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back(init_state);
    while !todo.is_empty() {
        let cur = todo.pop_front().expect("just checked");

        // treat any state with excess capacity or resources as the same
        // this is a huge (key) optimization
        if !seen.insert(State {
            time_remaining: cur.time_remaining, 
            ore: cur.ore.min(max_ore),
            clay: cur.clay.min(max_clay),
            obsidian: cur.obsidian.min(max_obsidian),
            geode: cur.geode,
            ore_bots: cur.ore_bots.min(max_ore),
            clay_bots: cur.clay_bots.min(max_clay),
            obsidian_bots: cur.obsidian_bots.min(max_obsidian),
            geode_bots: cur.geode_bots,
        }) {
            continue;
        }

        best = best.max(cur.geode);

        if cur.time_remaining == 0 {
            continue;
        }

        for resource in [
            Resource::Geode,
            Resource::Obsidian,
            Resource::Clay,
            Resource::Ore,
        ] {
            if let Some(new_state) = cur.buy_bot(&resource, &b) {
                todo.push_back(new_state);
            }
        }

        todo.push_back(State {
            time_remaining: cur.time_remaining - 1,
            ore: cur.ore + cur.ore_bots,
            clay: cur.clay + cur.clay_bots,
            obsidian: cur.obsidian + cur.obsidian_bots,
            geode: cur.geode + cur.geode_bots,
            ore_bots: cur.ore_bots,
            clay_bots: cur.clay_bots,
            obsidian_bots: cur.obsidian_bots,
            geode_bots: cur.geode_bots,
        });
    }

    dbg!(best)
}

fn part_1(blueprints: Vec<Blueprint>) -> usize {
    blueprints.iter().enumerate().map(|(i, b)| dbg!(i + 1) * get_max(b, 24)).sum()
}

fn part_2(blueprints: Vec<Blueprint>) -> usize {
    blueprints.iter().take(3).map(|b| get_max(b, 32)).product()
}


fn main() {
    assert_eq!(part_1(parse(include_str!("test.input.txt"))), 33);
    println!("Part 1: {}", part_1(parse(include_str!("input.txt"))));

    assert_eq!(get_max(&parse(include_str!("test.input.txt"))[0], 32), 56);
    assert_eq!(get_max(&parse(include_str!("test.input.txt"))[1], 32), 62);
    println!("Part 2: {}", part_2(parse(include_str!("input.txt"))));
}
