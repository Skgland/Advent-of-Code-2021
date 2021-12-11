use std::collections::HashSet;

fn parse_input(input: &str) -> [[u8; 10]; 10] {
    let mut lines = input.lines();
    [(); 10].map(|_| {
        let mut line = lines.next().unwrap().trim().chars();
        [(); 10].map(|_| line.next().unwrap() as u8 - b'0')
    })
}

pub fn neighbours(x: isize, y: isize, map: &[[u8; 10]; 10]) -> Vec<(usize, usize)> {
    let mut elems = vec![];

    for x_off in -1..=1 {
        for y_off in -1..=1 {
            let x_idx = x + x_off;
            let y_idx = y + y_off;
            if (x, y) != (x + x_off, y + y_off)
                && (0..map.len() as isize).contains(&x_idx)
                && (0..map[x_idx as usize].len() as isize).contains(&y_idx)
            {
                elems.push((x_idx as usize, y_idx as usize));
            }
        }
    }

    elems
}

fn iterate(input: &mut [[u8; 10]; 10]) -> usize {
    input
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|elem| *elem += 1);

    let mut to_flash = Vec::with_capacity(100);
    let mut flashed = HashSet::with_capacity(100);

    let ready = input.iter().enumerate().flat_map(|(x, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, &v)| v > 9)
            .map(move |(y, _)| (x, y))
    });

    to_flash.extend(ready);

    while let Some(elem) = to_flash.pop() {
        if flashed.insert(elem) {
            let mut neighbours = neighbours(elem.0 as isize, elem.1 as isize, &input);
            neighbours.iter().for_each(|&(x, y)| input[x][y] += 1);
            neighbours.retain(|elem| input[elem.0][elem.1] > 9);
            to_flash.extend(neighbours)
        }
    }

    input
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .filter(|elem| **elem > 9)
        .for_each(|elem| *elem = 0);

    flashed.len()
}

pub fn part1(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += iterate(&mut input);
    }
    flash_count
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse_input(input);
    let mut iteration_count = 0;

    loop {
        iteration_count += 1;
        if iterate(&mut input) == 100 {
            return iteration_count;
        }
    }
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day11.example.txt"));
    assert_eq!(part1(input), 1656);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day11.txt"));
    assert_eq!(part1(input), 1640);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day11.example.txt");
    assert_eq!(part2(input), 195);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day11.txt"));
    assert_eq!(part2(input), 312);
}
