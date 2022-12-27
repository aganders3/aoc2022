use std::collections::HashMap;
use std::iter::FromIterator;

fn parse(s: &str) -> (Vec<isize>, HashMap<usize, (usize, usize)>) {
    let nums: Vec<isize> = s.lines().map(|s| s.parse().unwrap()).collect();
    let idx: Vec<usize> = (0..nums.len()).collect();
    let mut prev: Vec<usize> = idx.clone();
    prev.rotate_right(1);
    let mut next: Vec<usize> = idx.clone();
    next.rotate_left(1);

    (
        nums,
        HashMap::from_iter(idx.into_iter().zip(prev.into_iter().zip(next.into_iter()))),
    )
}

#[derive(Debug)]
struct LL {
    map: HashMap<usize, (usize, usize)>,
    len: usize,
}

impl LL {
    fn shift(&mut self, i: usize, n: isize) {
        // remove the node
        let (old_prev, old_next) = self.map.remove(&i).expect("not in list?");
        let (old_prev_prev, _) = *self.map.get(&old_prev).expect("no old prev?");
        let (_, old_next_next) = *self.map.get(&old_next).expect("no old next?");
        self.map.insert(old_prev, (old_prev_prev, old_next));
        self.map.insert(old_next, (old_prev, old_next_next));

        // new location
        let new_prev = match n {
            n if n < 0 => {
                let n = n.abs() % (self.len as isize - 1);
                (0..n).fold(old_prev, |acc, _v| {
                    self.map.get(&acc).expect("can't chase rev").0
                })
            },
            n if n > 0 => {
                let n = (n - 1) % (self.len as isize - 1);
                (0..n).fold(old_next, |acc, _v| {
                    self.map.get(&acc).expect("can't chase fwd").1
                })
            }
            _ => old_prev,
        };

        // reinsert the node
        let (new_prev_prev, new_next) = *self.map.get(&new_prev).expect("no new prev?");
        let (_, new_next_next) = *self.map.get(&new_next).expect("no new next?");
        self.map.insert(i, (new_prev, new_next));
        self.map.insert(new_prev, (new_prev_prev, i));
        self.map.insert(new_next, (i, new_next_next));
    }
}

fn part_1(nums: &Vec<isize>, map: HashMap<usize, (usize, usize)>) -> isize {
    let mut list = LL {
        map,
        len: nums.len(),
    };
    nums.iter().enumerate().for_each(|(i, n)| {
        list.shift(i, *n);
    });

    let index_of_zero = nums.iter().position(|&x| x == 0).unwrap();
    dbg!(
        nums[(0..1000).fold(dbg!(index_of_zero), |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    ) + dbg!(
        nums[(0..2000).fold(index_of_zero, |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    ) + dbg!(
        nums[(0..3000).fold(index_of_zero, |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    )
}

fn part_2(nums: &Vec<isize>, map: HashMap<usize, (usize, usize)>) -> isize {
    let mut list = LL {
        map,
        len: nums.len(),
    };

    let nums = nums.iter().map(|n| n * 811589153).collect::<Vec<isize>>();

    for i in 0..10 {
        println!("Mixing {i}");
        nums.iter().enumerate().for_each(|(i, n)| {
            list.shift(i, *n);
        });
    }

    let index_of_zero = nums.iter().position(|&x| x == 0).unwrap();
    dbg!(
        nums[(0..1000).fold(dbg!(index_of_zero), |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    ) + dbg!(
        nums[(0..2000).fold(index_of_zero, |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    ) + dbg!(
        nums[(0..3000).fold(index_of_zero, |acc, _v| list
            .map
            .get(&acc)
            .expect("can't chase fwd")
            .1)]
    )
}

fn main() {
    // dbg!(parse(include_str!("test.input.txt")));
    let (nums, map) = parse(include_str!("test.input.txt"));
    assert_eq!(part_1(&nums, map), 3);

    let (nums, map) = parse(include_str!("input.txt"));
    let part_1_res = part_1(&nums, map);
    println!("Part 1 {}", part_1_res);
    assert_eq!(part_1_res, 7153);

    let (nums, map) = parse(include_str!("test.input.txt"));
    assert_eq!(part_2(&nums, map), 1623178306);

    let (nums, map) = parse(include_str!("input.txt"));
    let part_2_res = part_2(&nums, map);
    println!("Part 2 {}", part_2_res);
    assert_eq!(part_2_res, 6146976244822);
}
