use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(char_to_digit).collect())
        .collect()
}

fn char_to_digit(c: char) -> u8 {
    c as u8 - b'0'
}

pub fn get_neighbours(pos: (usize, usize), dim: usize) -> impl Iterator<Item = (usize, usize)> {
    let right = (pos.0.checked_add(1).filter(move |&x| x < dim), Some(pos.1));
    let left = (pos.0.checked_sub(1), Some(pos.1));
    let up = (Some(pos.0), pos.1.checked_add(1).filter(move |&y| y < dim));
    let down = (Some(pos.0), pos.1.checked_sub(1));
    [right, left, up, down]
        .into_iter()
        .flat_map(|elem| match elem {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        })
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct HeapEntry {
    cost: Reverse<u32>,
    pos: (usize, usize),
}

pub fn traverse(map: &Vec<Vec<u8>>, multiplier: usize) -> u32 {
    let mut candidates = HashMap::from([((0, 0), 0)]);
    let mut done = HashSet::new();
    let dim = map.len();
    let destination = (dim * multiplier - 1, dim * multiplier - 1);

    while let Some((pos, cost)) = candidates
        .iter()
        .min_by_key(|&(_key, value)| value)
        .map(|(&pos, &cost)| (pos, cost))
    {
        if pos == destination {
            return cost;
        }
        candidates.remove(&pos);
        done.insert(pos);
        get_neighbours(pos, map.len() * multiplier)
            .filter(|neighbor| !done.contains(neighbor))
            .for_each(|neighbor_pos| {
                let neighbour_cost = (map[neighbor_pos.0 % map.len()][neighbor_pos.1 % dim] as u32
                    + (neighbor_pos.0 / dim + neighbor_pos.1 / dim) as u32
                    - 1)
                    % 9
                    + 1;
                let total_cost = cost + neighbour_cost;
                let value = candidates.entry(neighbor_pos).or_insert(total_cost);
                *value = (*value).min(total_cost)
            });
    }
    panic!("Never reached the exit!");
}

pub fn part1(input: &str) -> u32 {
    let map = parse_input(input);
    traverse(&map, 1)
}

pub fn part2(input: &str) -> u32 {
    let map = parse_input(input);
    traverse(&map, 5)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day15.example.txt"));
    assert_eq!(part1(input), 40);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day15.txt"));
    assert_eq!(part1(input), 562);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day15.example.txt");
    assert_eq!(part2(input), 315);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day15.txt"));
    assert_eq!(part2(input), 2874);
}
