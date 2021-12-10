use crate::day10::Delimiter::{AngleBracket, Brace, Bracket, Parenthesis};
use crate::day10::Side::{Close, Open};

#[derive(Debug)]
enum Side {
    Open(Delimiter),
    Close(Delimiter),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Delimiter {
    Parenthesis,
    Bracket,
    Brace,
    AngleBracket,
}

impl Delimiter {
    pub fn score1(&self) -> usize {
        match self {
            Parenthesis => 3,
            Bracket => 57,
            Brace => 1197,
            AngleBracket => 25137,
        }
    }
    pub fn score2(&self) -> usize {
        match self {
            Parenthesis => 1,
            Bracket => 2,
            Brace => 3,
            AngleBracket => 4,
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = Side> + '_> + '_ {
    input.lines().map(|line| {
        line.chars().flat_map(|char| match char {
            '(' => Some(Open(Parenthesis)),
            '[' => Some(Open(Bracket)),
            '{' => Some(Open(Brace)),
            '<' => Some(Open(AngleBracket)),
            ')' => Some(Close(Parenthesis)),
            ']' => Some(Close(Bracket)),
            '}' => Some(Close(Brace)),
            '>' => Some(Close(AngleBracket)),
            _ => None,
        })
    })
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|line| {
            let mut stack = vec![];
            for delim in line {
                match delim {
                    Open(delim) => stack.push(delim),
                    Close(delim) => {
                        if Some(delim) != stack.pop() {
                            return delim.score1();
                        }
                    }
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut completions: Vec<_> = parse_input(input)
        .flat_map(|line| {
            let mut stack = vec![];
            for delim in line {
                match delim {
                    Open(delim) => stack.push(delim),
                    Close(delim) => {
                        if Some(delim) != stack.pop() {
                            return None;
                        }
                    }
                }
            }

            let res = stack
                .iter()
                .rev()
                .fold(0, |acc, delim| acc * 5 + delim.score2());
            if res == 0 {
                None
            } else {
                Some(res)
            }
        })
        .collect();
    completions.sort_unstable();
    completions[completions.len() / 2]
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day10.example.txt"));
    assert_eq!(part1(input), 2 * 3 + 57 + 1197 + 25137);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day10.txt"));
    assert_eq!(part1(input), 318081);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day10.example.txt");
    assert_eq!(part2(input), 288957);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day10.txt"));
    assert_eq!(part2(input), 4361305341);
}
