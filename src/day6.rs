use std::collections::VecDeque;

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .flat_map(|line| line.split(",").map(|elem| elem.parse().unwrap()))
}

pub fn both(input: &str, days: u32) -> usize {
    let mut aging_queue = VecDeque::with_capacity(9);
    aging_queue.resize(9, 0);
    let mut counter = 0;
    for elem in parse_input(input) {
        counter += 1;
        aging_queue[elem] += 1;
    }

    for day in 1..=days {
        let today = aging_queue.pop_front().unwrap();
        println!("Day {}: {}", day, today);
        counter += today;
        aging_queue[6] += today;
        aging_queue.push_back(today);
    }
    counter
}

///
///```rust
/// # use aoc2021::day6::part1;
/// let input = include_str!("../input/day6.example.txt");
///
/// assert_eq!(part1(input), 5934);
/// ```
///
pub fn part1(input: &str) -> usize {
    both(input, 80)
}

///
///```rust
/// # use aoc2021::day6::part2;
/// let input = include_str!("../input/day6.example.txt");
/// assert_eq!(part2(input), 26984457539);
/// ```
///
pub fn part2(input: &str) -> usize {
    both(input, 256)
}
