use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct Target {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

fn parse_input(input: &str) -> Target {
    let ranges = input.strip_prefix("target area: ").unwrap();
    let (prefixed_x_range, prefixed_y_range) = ranges.split_once(", ").unwrap();
    let (x_range_start, x_range_end) = prefixed_x_range
        .strip_prefix("x=")
        .unwrap()
        .split_once("..")
        .unwrap();
    let (y_range_start, y_range_end) = prefixed_y_range
        .strip_prefix("y=")
        .unwrap()
        .split_once("..")
        .unwrap();
    Target {
        x: x_range_start.parse().unwrap()..=x_range_end.parse().unwrap(),
        y: y_range_start.parse().unwrap()..=y_range_end.parse().unwrap(),
    }
}

/// For the initial y-Velocity return the maximum height reached
pub fn max_height(initial_y_velocity: i32) -> i32 {
    (0..=initial_y_velocity).sum()
}

/// return the steps (starting with step 0) after which we in the y-range of the target area
pub fn y_in_target_range(target: Target, initial_velocity: i32) -> impl IntoIterator<Item = i32> {
    let mut current_velocity = initial_velocity;

    let mut current_y = 0;
    let mut step = 0;

    let mut valid = vec![];

    while *target.y.start() <= current_y {
        current_y += current_velocity;
        current_velocity -= 1;
        if target.y.contains(&current_y) {
            valid.push(step);
        }
        step += 1;
    }
    valid
}

/// return the x-velocities that are in the targets x-Range after the specified step (starting with step 0)
pub fn reaching_x_velocities(target: Target, steps: i32) -> Vec<i32> {
    let min_x = 0;
    let max_x = *target.x.end();

    (min_x..=max_x)
        .into_iter()
        .filter(|initial_x| {
            target
                .x
                .contains(&(0..=steps).map(|step| 0.max(initial_x - step)).sum())
        })
        .collect()
}

/// return the vectors that at some point reach the target area
pub fn possible_vectors(target: Target) -> HashSet<(i32, i32)> {
    let min_y = 0.min(*target.y.start());
    let max_y = target.y.start().abs().max(target.y.end().abs());
    (min_y..=max_y)
        .into_iter()
        .flat_map(|y_velocity| {
            let step_range = y_in_target_range(target.clone(), y_velocity);
            step_range.into_iter().map(move |step| (y_velocity, step))
        })
        .flat_map(|(y_velocity, steps)| {
            let x_velocities = reaching_x_velocities(target.clone(), steps);
            x_velocities
                .into_iter()
                .map(move |x_velocity| (x_velocity, y_velocity))
        })
        .collect::<HashSet<_>>()
}

pub fn part1(input: &str) -> i32 {
    possible_vectors(parse_input(input))
        .into_iter()
        .map(|(_, y_vel)| max_height(y_vel))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    possible_vectors(parse_input(input)).len()
}

/// Simulates with the initial vector returning whether the target area is reach after any step
#[cfg(test)]
fn simulate(target: Target, init_vector: (i32, i32)) -> bool {
    let mut current_vector = init_vector;
    let mut current_x = 0;
    let mut current_y = 0;
    while current_x <= *target.x.end() && *target.y.start() <= current_y {
        current_x += current_vector.0;
        current_y += current_vector.1;
        current_vector.0 = 0.max(current_vector.0 - 1);
        current_vector.1 -= 1;
        if target.x.contains(&current_x) && target.y.contains(&current_y) {
            return true;
        }
    }
    false
}

#[test]
fn sanity_example() {
    let input = include_str!(concat!("../input/day17.example.txt"));
    let target = parse_input(input);
    let possible = possible_vectors(target.clone());
    assert!(possible
        .into_iter()
        .all(|elem| simulate(target.clone(), elem)))
}
#[test]
fn sanity_full() {
    let input = include_str!(concat!("../input/day17.txt"));
    let target = parse_input(input);
    let possible = possible_vectors(target.clone());
    assert!(possible
        .into_iter()
        .all(|elem| simulate(target.clone(), elem)))
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day17.example.txt"));
    assert_eq!(part1(input), 45);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day17.txt"));
    assert_eq!(part1(input), 2701);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day17.example.txt");
    assert_eq!(part2(input), 112);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day17.txt"));
    assert_eq!(part2(input), 1070);
}
