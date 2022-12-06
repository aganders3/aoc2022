use std::collections::HashSet;

fn find_marker(input: &str, marker_len: usize) -> usize {
    let mut unique = HashSet::new();
    marker_len
        + input.as_bytes()
            .windows(marker_len)
            .take_while(|x| {
                unique.clear();
                !x.iter().all(|y| unique.insert(y))
            })
            .count()
}

fn main() {
    // part 1
    assert!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4) == 7);
    assert!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4) == 5);
    assert!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4) == 6);
    assert!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4) == 10);
    assert!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4) == 11);
    let n = find_marker(include_str!("input.txt"), 4);
    println!("Part 1: {n}");

    // part 2
    assert!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14) == 19);
    assert!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14) == 23);
    assert!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14) == 23);
    assert!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14) == 29);
    assert!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14) == 26);
    let n = find_marker(include_str!("input.txt"), 14);
    println!("Part 2: {n}");
}
