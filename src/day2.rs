use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => return Err(()),
        })
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, i32)> + '_ {
    input.lines().map(|line| {
        let (dir, dist) = line.split_once(' ').unwrap();
        (dir.parse::<Direction>().unwrap(), dist.parse().unwrap())
    })
}

pub fn part1(input: &str) -> i32 {
    let iter = parse_input(input);

    let (depth, distance) = iter.fold((0, 0), |(depth, distance), (dir, dist)| match dir {
        Direction::Forward => (depth, distance + dist),
        Direction::Down => (depth + dist, distance),
        Direction::Up => (depth - dist, distance),
    });

    depth * distance
}

pub fn part2(input: &str) -> i32 {
    let iter = parse_input(input);

    let (_, depth, distance) =
        iter.fold((0, 0, 0), |(aim, depth, distance), (dir, dist)| match dir {
            Direction::Forward => (aim, depth + aim * dist, distance + dist),
            Direction::Down => (aim + dist, depth, distance),
            Direction::Up => (aim - dist, depth, distance),
        });

    depth * distance
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day2.example.txt");
    assert_eq!(part1(input), 10 * 15);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day2.txt"));
    assert_eq!(part1(input), 1882980);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day2.example.txt");
    assert_eq!(part2(input), 15 * 60);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day2.txt"));
    assert_eq!(part2(input), 1971232560);
}
