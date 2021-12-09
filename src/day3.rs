use std::cmp::Ordering;

fn parse_input(input: &str) -> impl Iterator<Item = &[u8]> + '_ {
    input.lines().map(|str| str.as_bytes())
}

/// Produces a bit mask with the lower n bits set
///```rust
/// # use aoc2021::day3::mask;
/// assert_eq!(mask(5), 0b11111);
///```
pub const fn mask(bits: usize) -> u32 {
    (1 << bits) - 1
}

pub fn part1(input: &str) -> u32 {
    let mut iter = parse_input(input).peekable();

    let bit_count = iter.peek().unwrap().len();
    let mut bit_vector = vec![0; bit_count];

    for bits in iter {
        for (idx, bit) in bits.iter().enumerate() {
            match bit {
                b'0' => bit_vector[idx] -= 1,
                b'1' => bit_vector[idx] += 1,
                _ => {}
            }
        }
    }

    let mut gamma = 0;

    for (idx, bit) in bit_vector.iter().enumerate() {
        gamma |= ((*bit > 0) as u32) << (bit_count - 1 - idx)
    }

    let epsilon = (!gamma) & mask(bit_count);

    gamma * epsilon
}

pub fn part2(input: &str) -> u32 {
    let mut input_list: Vec<_> = parse_input(input).collect();

    fn reduce_list(list: &mut Vec<&[u8]>, most: bool) -> u32 {
        let mut idx = 0;
        while list.len() > 1 {
            let filter = match list
                .iter()
                .map(|bits| bits[idx])
                .fold(0, |acc, next| match next {
                    b'0' => acc - 1,
                    b'1' => acc + 1,
                    _ => acc,
                })
                .cmp(&0)
            {
                Ordering::Less => {
                    if most {
                        b'0'
                    } else {
                        b'1'
                    }
                }
                Ordering::Equal | Ordering::Greater => {
                    if most {
                        b'1'
                    } else {
                        b'0'
                    }
                }
            };

            list.retain(|elem| elem[idx] == filter);
            idx += 1;
        }

        let mut result = 0;
        for (idx, bit) in list[0].iter().rev().enumerate() {
            if *bit == b'1' {
                result |= 1 << idx;
            }
        }
        result
    }

    let oxygen_generator = reduce_list(&mut input_list.clone(), true);
    let co2_scrubber = reduce_list(&mut input_list, false);

    oxygen_generator * co2_scrubber
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day3.example.txt");
    assert_eq!(part1(input), 22 * 9);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day3.txt"));
    assert_eq!(part1(input), 3242606);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day3.example.txt");
    assert_eq!(part2(input), 23 * 10);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day3.txt"));
    assert_eq!(part2(input), 4856080);
}
