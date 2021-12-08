use std::collections::VecDeque;

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .lines()
        .flat_map(|line| line.split(',').map(|elem| elem.parse().unwrap()))
}

pub fn both(input: &str, days: u32) -> usize {
    let mut aging_queue = VecDeque::with_capacity(9);
    aging_queue.resize(9, 0);
    let mut counter = 0;
    for elem in parse_input(input) {
        counter += 1;
        aging_queue[elem] += 1;
    }

    for _day in 1..=days {
        let today = aging_queue.pop_front().unwrap();
        counter += today;
        aging_queue[6] += today;
        aging_queue.push_back(today);
    }
    counter
}

pub fn part1(input: &str) -> usize {
    both(input, 80)
}

pub fn part2(input: &str) -> usize {
    both(input, 256)
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day6.example.txt");
    assert_eq!(part1(input), 5934);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day6.txt"));
    assert_eq!(part1(input), 349549);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day6.example.txt");
    assert_eq!(part2(input), 26984457539);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day6.txt"));
    assert_eq!(part2(input), 1589590444365);
}
