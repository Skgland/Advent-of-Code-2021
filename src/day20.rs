use std::fmt::{Formatter, Write};

pub struct Input {
    enhancement_alg: Vec<bool>,
    image: Vec<Vec<bool>>,
    defaul_pixel: bool,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.image {
            for entry in row {
                f.write_char(if *entry { '#' } else { '.' })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Input {
    fn get_pixel(&self, r: isize, c: isize) -> bool {
        if r >= 0 && c >= 0 {
            let r = r as usize;
            let c = c as usize;
            if let Some(row) = self.image.get(r) {
                if let Some(p) = row.get(c) {
                    return *p;
                }
            }
        }
        self.defaul_pixel
    }
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    fn char_to_pixel(c: char) -> bool {
        match c {
            '#' => true,
            '.' => false,
            _ => panic!("{} is not a valid pixel value", c),
        }
    }

    let enhancement_alg = lines.next().unwrap().chars().map(char_to_pixel).collect();

    let _ = lines.next();

    let image = lines
        .map(|line| line.chars().map(char_to_pixel).collect())
        .collect();

    Input {
        enhancement_alg,
        image,
        defaul_pixel: false,
    }
}

fn apply_enhancement(input: &mut Input) {
    let mut new_image = vec![vec![false; input.image[0].len() + 2]; input.image.len() + 2];

    for (idx_row, row) in new_image.iter_mut().enumerate() {
        for (idx_col, entry) in row.iter_mut().enumerate() {
            let mut lookup_idx = 0;
            for row in -1..=1 {
                for col in -1..=1 {
                    lookup_idx <<= 1;
                    lookup_idx |= input
                        .get_pixel(idx_row as isize + row - 1, idx_col as isize + col - 1)
                        as usize;
                }
            }
            *entry = input.enhancement_alg[lookup_idx];
        }
    }

    if input.defaul_pixel {
        input.defaul_pixel = input.enhancement_alg[0b111111111]
    } else {
        input.defaul_pixel = input.enhancement_alg[0b000000000]
    }

    input.image = new_image;
}

pub fn enhance(mut input: Input, iterations: usize) -> usize {
    for _ in 0..iterations {
        apply_enhancement(&mut input)
    }

    input
        .image
        .iter()
        .flat_map(|elem| elem.iter())
        .filter(|elem| **elem)
        .count()
}

pub fn part1(input: &str) -> usize {
    enhance(parse_input(input), 2)
}

pub fn part2(input: &str) -> usize {
    enhance(parse_input(input), 50)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day20.example.txt"));
    assert_eq!(part1(input), 35);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day20.txt"));
    assert_eq!(part1(input), 5400);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day20.example.txt");
    assert_eq!(part2(input), 3351);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day20.txt"));
    assert_eq!(part2(input), 18989);
}
