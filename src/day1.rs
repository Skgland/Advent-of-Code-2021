use std::collections::VecDeque;

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn both(input: &str, window_size: usize) -> u32 {
    let mut iter = parse_input(input);

    let mut window = VecDeque::with_capacity(window_size);

    window.extend((&mut iter).take(window_size));

    let mut counter = 0;

    for current in iter {
        let last = window.pop_front().unwrap();
        window.push_back(current);

        if current > last {
            counter += 1;
        }
    }

    counter
}

pub fn part1(input: &str) -> u32 {
    both(input, 1)
}

pub fn part2(input: &str) -> u32 {
    both(input, 3)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day1.example.txt"));
    assert_eq!(part1(input), 7);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day1.txt"));
    assert_eq!(part1(input), 1292);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day1.example.txt");
    assert_eq!(part2(input), 5);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day1.txt"));
    assert_eq!(part2(input), 1262);
}
