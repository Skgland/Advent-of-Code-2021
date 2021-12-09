#![allow(clippy::ptr_arg)]

use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.chars().map(|char| char as u8 - b'0').collect())
        .collect()
}

pub fn neighbours<T: Copy>(
    x: usize,
    y: usize,
    map: &Vec<Vec<T>>,
) -> impl Iterator<Item = (usize, usize, T)> {
    let a = x.checked_sub(1).and_then(|x| {
        map.get(x)
            .map(|elem| (x, elem))
            .and_then(|(x, row)| row.get(y).map(|&elem| (x, y, elem)))
    });
    let b = x.checked_add(1).and_then(|x| {
        map.get(x)
            .map(|elem| (x, elem))
            .and_then(|(x, row)| row.get(y).map(|&elem| (x, y, elem)))
    });
    let c = y
        .checked_sub(1)
        .zip(map.get(x))
        .and_then(|(y, row)| row.get(y).map(|&elem| (x, y, elem)));
    let d = y
        .checked_add(1)
        .zip(map.get(x))
        .and_then(|(y, row)| row.get(y).map(|&elem| (x, y, elem)));
    [a, b, c, d].into_iter().flatten()
}

pub fn positions<T: Copy>(map: &Vec<Vec<T>>) -> impl Iterator<Item = (usize, usize, T)> + '_ {
    map.iter().enumerate().flat_map(|(x_idx, row)| {
        row.iter()
            .copied()
            .enumerate()
            .map(move |(y_idx, elem)| (x_idx, y_idx, elem))
    })
}

pub fn low(map: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
    positions(map).filter(|&(x, y, value)| neighbours(x, y, map).all(|(_, _, n)| n > value))
}

pub fn basin_size(x: usize, y: usize, depth: u8, map: &Vec<Vec<u8>>) -> usize {
    let mut todo = VecDeque::from([(x, y, depth)]);
    let mut processed = HashSet::new();
    while let Some(pos) = todo.pop_front() {
        if processed.insert((pos.0, pos.1)) {
            todo.extend(neighbours(pos.0, pos.1, map).filter(|&(_, _, d)| d > pos.2 && d != 9))
        }
    }
    processed.len()
}

pub fn part1(input: &str) -> u32 {
    let depth_map = parse_input(input);
    low(&depth_map).map(|(_, _, v)| v as u32 + 1).sum()
}

pub fn part2(input: &str) -> usize {
    let depth_map = parse_input(input);

    let mut basins = low(&depth_map)
        .map(|(x, y, value)| basin_size(x, y, value, &depth_map))
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins.reverse();
    basins.iter().take(3).product()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day9.example.txt"));
    assert_eq!(part1(input), 2 + 1 + 6 + 6);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day9.txt"));
    assert_eq!(part1(input), 480);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day9.example.txt");
    assert_eq!(part2(input), 9 * 14 * 9);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day9.txt"));
    assert_eq!(part2(input), 1045660);
}
