use std::collections::HashMap;

struct Line {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

impl Line {
    fn x_diff(&self) -> u32 {
        self.start_x.max(self.end_x) - self.start_x.min(self.end_x)
    }
    fn y_diff(&self) -> u32 {
        self.start_y.max(self.end_y) - self.start_y.min(self.end_y)
    }
    fn length(&self) -> u32 {
        self.x_diff().max(self.y_diff())
    }

    fn point(&self, idx: u32) -> (u32, u32) {
        let x_offset = idx * self.x_diff() / self.length();
        let y_offset = idx * self.y_diff() / self.length();
        (
            if self.start_x > self.end_x {
                self.start_x - x_offset
            } else {
                self.start_x + x_offset
            },
            if self.start_y > self.end_y {
                self.start_y - y_offset
            } else {
                self.start_y + y_offset
            },
        )
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().map(|line| {
        let mut pairs = line.split(" -> ");
        let start_pair = pairs.next().unwrap();
        let end_pair = pairs.next().unwrap();
        let mut starts = start_pair.split(',');
        let mut ends = end_pair.split(',');
        Line {
            start_x: starts.next().unwrap().parse().unwrap(),
            start_y: starts.next().unwrap().parse().unwrap(),
            end_x: ends.next().unwrap().parse().unwrap(),
            end_y: ends.next().unwrap().parse().unwrap(),
        }
    })
}

pub fn both(input: &str, filter: bool) -> usize {
    parse_input(input)
        .filter(|line| line.start_x == line.end_x || line.start_y == line.end_y || !filter)
        .flat_map(|line| {
            let max_diff = line.length();
            (0..=max_diff).map(move |offset| line.point(offset))
        })
        .fold(HashMap::new(), |mut acc: HashMap<(_, _), bool>, next| {
            acc.entry(next)
                .and_modify(|value| *value = true)
                .or_insert(false);
            acc
        })
        .iter()
        .filter(|&(&_, &value)| value)
        .count()
}

pub fn part1(input: &str) -> usize {
    both(input, true)
}

pub fn part2(input: &str) -> usize {
    both(input, false)
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day5.example.txt");
    assert_eq!(part1(input), 5);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day5.txt"));
    assert_eq!(part1(input), 5084);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day5.example.txt");
    assert_eq!(part2(input), 12);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day5.txt"));
    assert_eq!(part2(input), 17882);
}
