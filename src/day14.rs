use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    pairs: HashMap<(char, char), u64>,
    char_counts: HashMap<char, u64>,
    mappings: HashMap<(char, char), char>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let sequence = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mappings = lines
        .skip(1)
        .flat_map(|line| line.split_once(" -> "))
        .map(|(pat, ins)| {
            let mut pat_chars = pat.chars();
            (
                (pat_chars.next().unwrap(), pat_chars.next().unwrap()),
                ins.chars().next().unwrap(),
            )
        })
        .collect();
    let char_counts = sequence.iter().fold(HashMap::new(), |mut map, elem| {
        *map.entry(*elem).or_default() += 1;
        map
    });
    let pairs = sequence.windows(2).fold(HashMap::new(), |mut map, elem| {
        match elem {
            [c1, c2] => {
                *map.entry((*c1, *c2)).or_default() += 1;
            }
            _ => {}
        }
        map
    });
    Input {
        pairs,
        char_counts,
        mappings,
    }
}

pub fn apply_mapping(input: &mut Input) {
    let pairs = input
        .pairs
        .iter()
        .flat_map(|(key, count)| {
            if let Some(insert) = input.mappings.get(key) {
                *input.char_counts.entry(*insert).or_default() += *count;
                vec![((key.0, *insert), *count), ((*insert, key.1), *count)]
            } else {
                vec![(*key, *count)]
            }
        })
        .fold(HashMap::new(), |mut map, (key, count)| {
            *map.entry(key).or_default() += count;
            map
        });

    input.pairs = pairs;
}

pub fn both(input: &str, iterations: u32) -> u64 {
    let mut input = parse_input(input);
    for _ in 0..iterations {
        apply_mapping(&mut input);
    }

    let min = input.char_counts.values().min().unwrap();
    let max = input.char_counts.values().max().unwrap();
    max - min
}

pub fn part1(input: &str) -> u64 {
    both(input, 10)
}

pub fn part2(input: &str) -> u64 {
    both(input, 40)
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day14.example.txt"));
    assert_eq!(part1(input), 1749 - 161);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day14.txt"));
    assert_eq!(part1(input), 2947);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day14.example.txt");
    assert_eq!(part2(input), 2188189693529);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day14.txt"));
    assert_eq!(part2(input), 3232426226464);
}
