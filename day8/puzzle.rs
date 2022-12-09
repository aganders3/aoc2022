fn part_1(s: &str) -> usize {
    let forest = s
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut visible = vec![vec![false; forest[0].len()]; forest.len()];

    let m = forest.len();
    let n = forest[0].len();

    for i in 0..m {
        for j in 0..n {
            let h = forest[i][j];
            // direction      up       left     right   down
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (mut next_i, mut next_j) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                let mut blocked = false;
                while !visible[i][j] && (0..m).contains(&next_i) && (0..n).contains(&next_j) {
                    if forest[next_i][next_j] >= h {
                        // next tree as tall or taller, note as blocked and stop looking in this
                        // direction
                        blocked = true;
                        break;
                    }
                    (next_i, next_j) = ((next_i as i32 + di) as usize, (next_j as i32 + dj) as usize);
                }
                // not blocked in this direction - mark visible and stop looking
                if !blocked { visible[i][j] = true;  break; }
            }
        }
    }

    visible.iter().flatten().filter(|x| **x).count()
}

fn part_2(s: &str) -> usize {
    let forest = s
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut score = vec![vec![1; forest[0].len()]; forest.len()];

    let m = forest.len();
    let n = forest[0].len();

    for i in 0..m {
        for j in 0..n {
            let h = forest[i][j];
            // direction      up       left     right   down
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (mut next_i, mut next_j) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
                let mut visible = 0;
                while (0..m).contains(&next_i) && (0..n).contains(&next_j) {
                    visible += 1;
                    if forest[next_i][next_j] >= h { break; }
                    (next_i, next_j) = ((next_i as i32 + di) as usize, (next_j as i32 + dj) as usize);
                }
                score[i][j] *= visible;
            }
        }
    }
    dbg!(*score.iter().flatten().max().unwrap()) as usize
}

fn main() {
    assert!(dbg!(part_1(include_str!("test.input.txt"))) == 21);
    println!("Part 1: {}", part_1(include_str!("input.txt")));

    assert!(dbg!(part_2(include_str!("test.input.txt"))) == 8);
    println!("Part 2: {}", part_2(include_str!("input.txt")));
}
